//! Build script for `litert-lm-rust`.
//!
//! Linking is controlled by environment variables:
//! - `LITERT_LM_LIB_DIR` — directory containing the LiteRT-LM C shared library
//! - `LITERT_LM_LIB_NAME` — library name without `lib` prefix / extension
//!   (default: `LiteRtLmC` on most platforms, `engine` as a common Bazel alias)
//! - `LITERT_LM_INCLUDE_DIR` — optional override for the C header directory
//!
//! Enable the `bindgen` feature to regenerate bindings from `c/wrapper.h`.

use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let _include_dir = env::var_os("LITERT_LM_INCLUDE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join("c"));

    println!("cargo:rerun-if-changed=c/engine.h");
    println!("cargo:rerun-if-changed=c/wrapper.h");
    println!("cargo:rerun-if-changed=src/bindings.rs");
    println!("cargo:rerun-if-env-changed=LITERT_LM_LIB_DIR");
    println!("cargo:rerun-if-env-changed=LITERT_LM_LIB_NAME");
    println!("cargo:rerun-if-env-changed=LITERT_LM_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=LITERT_LM_STATIC");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_dir.join("bindings.rs");

    #[cfg(feature = "bindgen")]
    {
        let include_dir = env::var_os("LITERT_LM_INCLUDE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| manifest_dir.join("c"));
        generate_bindings(&include_dir, &bindings_path);
    }

    #[cfg(not(feature = "bindgen"))]
    {
        let vendored = manifest_dir.join("src/bindings.rs");
        if vendored.exists() {
            std::fs::copy(&vendored, &bindings_path).expect("copy vendored bindings");
        } else {
            panic!(
                "src/bindings.rs missing. Build once with `--features bindgen` to generate it."
            );
        }
    }

    if env::var_os("CARGO_FEATURE_DOCS_ONLY").is_some() {
        return;
    }

    link_native_library(&manifest_dir);
}

#[cfg(feature = "bindgen")]
fn generate_bindings(include_dir: &PathBuf, bindings_path: &PathBuf) {
    let wrapper = include_dir.join("wrapper.h");
    let bindings = bindgen::Builder::default()
        .header(wrapper.to_string_lossy())
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg("-DLITERT_LM_C_API_EXPORT=")
        .allowlist_function("litert_lm_.*")
        .allowlist_type("LiteRtLm.*")
        .allowlist_var("kLiteRtLm.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate LiteRT-LM bindings");
    bindings
        .write_to_file(bindings_path)
        .expect("Couldn't write bindings");

    // Keep a checked-in copy for docs.rs / consumers without libclang.
    let vendored = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/bindings.rs");
    let _ = std::fs::copy(bindings_path, vendored);
}

fn link_native_library(manifest_dir: &PathBuf) {
    let lib_name = env::var("LITERT_LM_LIB_NAME").unwrap_or_else(|_| {
        // Common names: Bazel `engine`, community prebuilts `LiteRtLmC`.
        if cfg!(windows) {
            "engine".to_string()
        } else {
            "LiteRtLmC".to_string()
        }
    });

    let candidates = [
        env::var_os("LITERT_LM_LIB_DIR").map(PathBuf::from),
        Some(manifest_dir.join("c")),
        Some(manifest_dir.join("native")),
        Some(manifest_dir.join("prebuilt")),
    ];

    let mut found_dir = None;
    for candidate in candidates.into_iter().flatten() {
        if dir_has_library(&candidate, &lib_name) {
            found_dir = Some(candidate);
            break;
        }
        // Also try alternate common names in the same folder.
        for alt in ["LiteRtLmC", "engine", "LiteRtLm", "litert_lm_c"] {
            if alt != lib_name && dir_has_library(&candidate, alt) {
                println!("cargo:rustc-link-search=native={}", candidate.display());
                link_lib(alt);
                println!(
                    "cargo:warning=Linked `{alt}` from {} (override with LITERT_LM_LIB_NAME)",
                    candidate.display()
                );
                return;
            }
        }
    }

    if let Some(dir) = found_dir {
        println!("cargo:rustc-link-search=native={}", dir.display());
        link_lib(&lib_name);
        // On Windows, ensure the DLL can be found next to the binary at runtime.
        #[cfg(windows)]
        {
            println!("cargo:rustc-env=PATH={};{}", dir.display(), env::var("PATH").unwrap_or_default());
        }
        return;
    }

    println!(
        "cargo:warning=LiteRT-LM native library not found. Set LITERT_LM_LIB_DIR to the folder \
         containing lib{lib_name}.so / {lib_name}.dll / lib{lib_name}.dylib before linking an \
         executable. The Rust crate still compiles; only final linking needs the native library."
    );
    // Still emit a link directive so dependents know what to provide.
    link_lib(&lib_name);
}

fn link_lib(name: &str) {
    if env::var_os("LITERT_LM_STATIC").is_some() {
        println!("cargo:rustc-link-lib=static={name}");
    } else {
        println!("cargo:rustc-link-lib=dylib={name}");
    }
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=m");
        println!("cargo:rustc-link-lib=dylib=dl");
        println!("cargo:rustc-link-lib=dylib=pthread");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
    }
}

fn dir_has_library(dir: &PathBuf, name: &str) -> bool {
    if !dir.is_dir() {
        return false;
    }
    let names = [
        format!("lib{name}.so"),
        format!("lib{name}.dylib"),
        format!("{name}.dll"),
        format!("lib{name}.dll"),
        format!("{name}.lib"),
        format!("lib{name}.a"),
        format!("{name}.a"),
    ];
    names.iter().any(|n| dir.join(n).exists())
}
