use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let lvgl_dir = manifest_dir.join("lvgl");
    let lv_conf_dir = manifest_dir.join("lv_conf");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Allow custom lv_conf.h via environment variable
    let lv_conf_include = env::var("DEP_LV_CONF_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| lv_conf_dir.clone());

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=lv_conf/lv_conf.h");
    println!("cargo:rerun-if-env-changed=DEP_LV_CONF_PATH");
    println!("cargo:rerun-if-env-changed=ESP_TOOLCHAIN_VERSION");

    // Collect LVGL source files
    let src_dir = lvgl_dir.join("src");
    let mut sources: Vec<PathBuf> = Vec::new();

    collect_c_files(&src_dir, &mut sources);

    // Compile LVGL
    let mut build = cc::Build::new();
    build
        .include(&lvgl_dir)
        .include(&lv_conf_include)
        .define("LV_CONF_INCLUDE_SIMPLE", None)
        // Suppress warnings from LVGL code
        .warnings(false)
        .extra_warnings(false);

    // Add platform-specific flags
    let target = env::var("TARGET").unwrap_or_default();
    if target.contains("thumb") || target.contains("riscv") || target.contains("xtensa") {
        // Embedded targets - optimize for size
        build.opt_level_str("s");
    }

    // Xtensa requires -mlongcalls for large code (LVGL is big)
    if target.contains("xtensa") {
        build.flag("-mlongcalls");
    }

    // For ESP-IDF targets, we need to use the ESP toolchain compiler
    if target.contains("espidf") {
        // Try multiple ways to find the cross-compiler
        let compiler = find_esp_compiler(&target);
        if let Some(cc) = compiler {
            eprintln!("neo-lvgl-sys: Using compiler: {}", cc);
            build.compiler(&cc);

            // Also set the archiver
            let ar = cc.replace("-gcc", "-ar");
            if std::path::Path::new(&ar).exists() {
                build.archiver(&ar);
            }
        } else {
            eprintln!("neo-lvgl-sys: WARNING: Could not find ESP cross-compiler");
        }
    }

    for source in &sources {
        build.file(source);
    }

    build.compile("lvgl");

    // Explicitly emit link directives (cc crate should do this, but ESP-IDF cross-compilation
    // can have issues finding the library)
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=lvgl");

    // Debug: print info about what was built
    let lib_path = out_dir.join("liblvgl.a");
    if lib_path.exists() {
        eprintln!("neo-lvgl-sys: Built liblvgl.a at {}", lib_path.display());
    } else {
        eprintln!(
            "neo-lvgl-sys: WARNING: liblvgl.a not found at {}",
            lib_path.display()
        );
    }
    eprintln!("neo-lvgl-sys: TARGET={}", target);
    if let Ok(cc) = env::var("TARGET_CC") {
        eprintln!("neo-lvgl-sys: TARGET_CC={}", cc);
    }

    // Generate bindings
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", lvgl_dir.display()))
        .clang_arg(format!("-I{}", lv_conf_include.display()))
        .clang_arg("-DLV_CONF_INCLUDE_SIMPLE")
        // no_std compatibility
        .use_core()
        .ctypes_prefix("cty")
        // Allowlist LVGL API
        .allowlist_function("lv_.*")
        .allowlist_type("lv_.*")
        .allowlist_type("_lv_.*")
        .allowlist_var("LV_.*")
        // Block problematic types
        .blocklist_type("max_align_t")
        // Layout hints
        .layout_tests(false)
        .derive_default(true)
        .derive_debug(false);

    // Add target-specific clang arguments
    let target = env::var("TARGET").unwrap_or_default();
    if target.contains("apple") {
        // For macOS/iOS, set the target explicitly
        builder = builder.clang_arg(format!("--target={}", target));
    } else if target.contains("thumb") {
        // For ARM embedded, use appropriate target
        builder = builder
            .clang_arg("--target=arm-none-eabi")
            .clang_arg("-mthumb");
    } else if target.contains("riscv") && !target.contains("espidf") {
        builder = builder.clang_arg(format!("--target={}", target));
    } else if target.contains("xtensa") || target.contains("espidf") {
        // For ESP-IDF targets (Xtensa or RISC-V based ESP32)
        // We need to find the ESP toolchain sysroot for libc headers
        builder = configure_espidf_bindgen(builder, &target);
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Export the config path for dependent crates
    println!("cargo:root={}", lv_conf_include.display());
}

/// Configure bindgen for ESP-IDF targets
fn configure_espidf_bindgen(builder: bindgen::Builder, target: &str) -> bindgen::Builder {
    // ESP-IDF uses newlib, we need to find the toolchain's sysroot
    // The toolchain can be in multiple locations:
    // 1. ~/.rustup/toolchains/esp/xtensa-esp-elf/ (esp-rs toolchain)
    // 2. ~/.espressif/tools/ (standalone ESP-IDF)

    let home = env::var("HOME").unwrap_or_default();
    let home_path = PathBuf::from(&home);

    // Determine which toolchain to use based on target
    let toolchain_name = if target.contains("xtensa") {
        "xtensa-esp-elf"
    } else if target.contains("riscv") {
        "riscv32-esp-elf"
    } else {
        "xtensa-esp-elf"
    };

    // Check for user-specified toolchain version via environment variable
    let requested_version = env::var("ESP_IDF_VERSION").ok();

    // Try multiple possible locations for the toolchain
    let possible_bases = [
        // esp-rs Rust toolchain location
        home_path
            .join(".rustup/toolchains/esp")
            .join(toolchain_name),
        // Standalone ESP-IDF location
        home_path.join(".espressif/tools").join(toolchain_name),
    ];

    let sysroot = possible_bases
        .iter()
        .find_map(|base| find_esp_sysroot(base, toolchain_name, requested_version.as_deref()));

    let mut builder = builder
        // Use a 32-bit target that clang understands for parsing
        .clang_arg("--target=i686-unknown-linux-gnu")
        .clang_arg("-D__xtensa__")
        .clang_arg("-D__ESP_IDF__");

    if let Some(sysroot) = sysroot {
        builder = builder
            .clang_arg(format!("--sysroot={}", sysroot.display()))
            .clang_arg(format!("-I{}/include", sysroot.display()));
    } else {
        // Fallback: freestanding mode with manually defined types
        builder = builder
            .clang_arg("-ffreestanding")
            .clang_arg("-nostdinc")
            .clang_arg("-Dint8_t=signed char")
            .clang_arg("-Duint8_t=unsigned char")
            .clang_arg("-Dint16_t=short")
            .clang_arg("-Duint16_t=unsigned short")
            .clang_arg("-Dint32_t=int")
            .clang_arg("-Duint32_t=unsigned int")
            .clang_arg("-Dint64_t=long long")
            .clang_arg("-Duint64_t=unsigned long long")
            .clang_arg("-Dsize_t=unsigned int")
            .clang_arg("-Dptrdiff_t=int")
            .clang_arg("-Dintptr_t=int")
            .clang_arg("-Duintptr_t=unsigned int")
            .clang_arg("-DINT8_MIN=-128")
            .clang_arg("-DINT8_MAX=127")
            .clang_arg("-DUINT8_MAX=255")
            .clang_arg("-DINT16_MIN=-32768")
            .clang_arg("-DINT16_MAX=32767")
            .clang_arg("-DUINT16_MAX=65535")
            .clang_arg("-DINT32_MIN=-2147483648")
            .clang_arg("-DINT32_MAX=2147483647")
            .clang_arg("-DUINT32_MAX=4294967295U")
            .clang_arg("-DSIZE_MAX=4294967295U")
            .clang_arg("-D_STDINT_H")
            .clang_arg("-D_INTTYPES_H");
    }

    builder
}

/// Find the ESP toolchain sysroot
fn find_esp_sysroot(
    toolchain_base: &PathBuf,
    toolchain_name: &str,
    requested_version: Option<&str>,
) -> Option<PathBuf> {
    if !toolchain_base.exists() {
        return None;
    }

    let entries = std::fs::read_dir(toolchain_base).ok()?;
    let mut versions: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();

    // Sort by name (versions) descending to get latest first
    versions.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

    // If a specific version is requested, try to find it first
    if let Some(req_ver) = requested_version {
        for version_entry in &versions {
            let name = version_entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.contains(req_ver) {
                if let Some(sysroot) = try_find_sysroot_in_version(version_entry, toolchain_name) {
                    return Some(sysroot);
                }
            }
        }
    }

    // Otherwise, use the latest available version
    for version_entry in versions {
        if let Some(sysroot) = try_find_sysroot_in_version(&version_entry, toolchain_name) {
            return Some(sysroot);
        }
    }

    None
}

/// Try to find sysroot within a version directory
fn try_find_sysroot_in_version(
    version_entry: &std::fs::DirEntry,
    toolchain_name: &str,
) -> Option<PathBuf> {
    // Try different possible sysroot structures:
    // The actual newlib headers are typically at:
    // <version>/xtensa-esp-elf/xtensa-esp-elf/include/stdint.h
    let candidates = [
        // Most common: nested directories
        version_entry
            .path()
            .join(toolchain_name)
            .join(toolchain_name),
        // Alternative: single level
        version_entry.path().join(toolchain_name),
    ];

    for sysroot in candidates {
        // Check for actual newlib header, not just any include directory
        if sysroot.join("include/stdint.h").exists() {
            return Some(sysroot);
        }
    }

    None
}

/// Find the ESP cross-compiler for the given target
fn find_esp_compiler(target: &str) -> Option<String> {
    // 1. Check TARGET_CC environment variable first
    if let Ok(cc) = env::var("TARGET_CC") {
        if std::path::Path::new(&cc).exists() {
            return Some(cc);
        }
    }

    // 2. Check CC_<target> environment variable
    let target_env = target.replace('-', "_");
    if let Ok(cc) = env::var(format!("CC_{}", target_env)) {
        if std::path::Path::new(&cc).exists() {
            return Some(cc);
        }
    }

    // Determine compiler prefix based on target
    let prefix = if target.contains("xtensa-esp32s3") {
        "xtensa-esp32s3-elf"
    } else if target.contains("xtensa-esp32s2") {
        "xtensa-esp32s2-elf"
    } else if target.contains("xtensa-esp32") {
        "xtensa-esp32-elf"
    } else if target.contains("riscv32") {
        "riscv32-esp-elf"
    } else {
        "xtensa-esp-elf" // generic fallback
    };

    let gcc_name = format!("{}-gcc", prefix);

    // 3. Check if it's in PATH
    if let Ok(output) = std::process::Command::new("which").arg(&gcc_name).output() {
        if output.status.success() {
            if let Ok(path) = String::from_utf8(output.stdout) {
                let path = path.trim();
                if !path.is_empty() {
                    return Some(path.to_string());
                }
            }
        }
    }

    // 4. Look in common ESP-IDF toolchain locations
    let home = env::var("HOME").unwrap_or_default();

    // Check for .embuild directory (used by esp-idf-sys)
    // Structure: .embuild/espressif/tools/xtensa-esp-elf/<version>/xtensa-esp-elf/bin/
    if let Ok(embuild_dir) = env::var("ESP_IDF_TOOLS_INSTALL_DIR") {
        if let Some(cc) = find_compiler_in_embuild(&PathBuf::from(embuild_dir), prefix) {
            return Some(cc);
        }
    }

    // Also check OUT_DIR parent directories for .embuild
    if let Ok(out_dir) = env::var("OUT_DIR") {
        let mut path = PathBuf::from(&out_dir);
        // Walk up to find the project root
        for _ in 0..10 {
            let embuild = path.join(".embuild/espressif/tools");
            if embuild.exists() {
                if let Some(cc) = find_compiler_in_embuild(&embuild, prefix) {
                    return Some(cc);
                }
            }
            if !path.pop() {
                break;
            }
        }
    }

    // Check ~/.espressif/tools
    let espressif_tools = PathBuf::from(&home).join(".espressif/tools");
    if let Some(cc) = find_compiler_in_embuild(&espressif_tools, prefix) {
        return Some(cc);
    }

    None
}

/// Find compiler in embuild/espressif tools directory structure
fn find_compiler_in_embuild(tools_dir: &PathBuf, prefix: &str) -> Option<String> {
    // Try both xtensa-esp-elf (generic) and specific prefixes
    let toolchain_names = ["xtensa-esp-elf", prefix];

    for toolchain_name in &toolchain_names {
        let toolchain_dir = tools_dir.join(toolchain_name);
        if !toolchain_dir.exists() {
            continue;
        }

        // Find version directories
        if let Ok(entries) = std::fs::read_dir(&toolchain_dir) {
            let mut versions: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            // Sort descending to get latest version first
            versions.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

            for version_entry in versions {
                // The compiler is at: <version>/xtensa-esp-elf/bin/xtensa-esp-elf-gcc
                // or: <version>/xtensa-esp32s3-elf/bin/xtensa-esp32s3-elf-gcc
                let bin_paths = [
                    version_entry.path().join("xtensa-esp-elf/bin"),
                    version_entry.path().join(format!("{}/bin", prefix)),
                    version_entry.path().join("bin"),
                ];

                for bin_path in &bin_paths {
                    let gcc = bin_path.join(format!("{}-gcc", prefix));
                    if gcc.exists() {
                        return Some(gcc.to_string_lossy().to_string());
                    }
                    // Also try generic xtensa-esp-elf-gcc
                    let gcc = bin_path.join("xtensa-esp-elf-gcc");
                    if gcc.exists() {
                        return Some(gcc.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    None
}

fn collect_c_files(dir: &PathBuf, sources: &mut Vec<PathBuf>) {
    if !dir.exists() {
        return;
    }

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // Skip demo and example directories
            let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if dir_name == "demos" || dir_name == "examples" {
                continue;
            }
            collect_c_files(&path, sources);
        } else if path.extension().map_or(false, |ext| ext == "c") {
            sources.push(path);
        }
    }
}
