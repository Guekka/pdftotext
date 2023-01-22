#[cfg(feature = "static-poppler")]
fn setup(callpoppler: &mut cc::Build) -> &mut cc::Build {
    let static_build = cmake::Config::new("poppler")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_GTK_TESTS", "OFF")
        .define("BUILD_QT5_TESTS", "OFF")
        .define("BUILD_QT6_TESTS", "OFF")
        .define("BUILD_CPP_TESTS", "OFF")
        .define("ENABLE_UNSTABLE_API_ABI_HEADERS", "ON")
        .define("ENABLE_SPLASH", "OFF")
        .define("ENABLE_UTILS", "OFF")
        .define("ENABLE_GLIB", "OFF")
        .define("ENABLE_GOBJECT_INTROSPECTION", "OFF")
        .define("ENABLE_CPP", "OFF")
        .define("ENABLE_GTK_DOC", "OFF")
        .define("ENABLE_QT5", "OFF")
        .define("ENABLE_QT6", "OFF")
        .define("ENABLE_LIBOPENJPEG", "unmaintained")
        .define("ENABLE_CMS", "none")
        .define("ENABLE_LIBCURL", "OFF")
        .define("ENABLE_ZLIB", "OFF")
        .define("ENABLE_DCTDECODER", "unmaintained")
        .define("ENABLE_ZLIB_UNCOMPRESS", "OFF")
        .define("SPLASH_CMYK", "OFF")
        .define("WITH_JPEG", "OFF")
        .define("WITH_PNG", "OFF")
        .define("WITH_TIFF", "OFF")
        .define("WITH_NSS3", "OFF")
        .define("WITH_Cairo", "OFF")
        .define("WITH_FONTCONFIGURATION_FONTCONFIG", "OFF")
        .define("RUN_GPERF_IF_PRESENT", "OFF")
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build",
        static_build.display()
    );
    println!("cargo:rustc-link-lib=static=poppler");

    let dir = std::path::Path::new(&static_build).join("include/poppler");

    let dir_config = std::path::Path::new(&static_build).join("build/poppler");
    let callpoppler = callpoppler
        .include(dir)
        .include(dir_config)
        .flag_if_supported("-std=c++20");

    callpoppler
}

#[cfg(feature = "static-poppler-wasm")]
fn setup(mut callpoppler: &mut cc::Build) -> &mut cc::Build {
    let static_build = cmake::Config::new("poppler")
        .cflag("-s USE_FREETYPE=1")
        .cxxflag("-s USE_FREETYPE=1")
        // manually set until i can figure out how to automatically get it
        .define("CMAKE_TOOLCHAIN_FILE", "/Users/rileysmith/repos/emsdk/upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake")
        .define("CMAKE_CROSSCOMPILING_EMULATOR", "/Users/rileysmith/repos/emsdk/node/14.18.2_64bit/bin/node;--experimental-wasm-threads")
        .define("CMAKE_VERBOSE_MAKEFILE:BOOL", "ON")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_GTK_TESTS", "OFF")
        .define("BUILD_QT5_TESTS", "OFF")
        .define("BUILD_QT6_TESTS", "OFF")
        .define("BUILD_CPP_TESTS", "OFF")
        .define("ENABLE_UNSTABLE_API_ABI_HEADERS", "ON")
        .define("ENABLE_SPLASH", "OFF")
        .define("ENABLE_UTILS", "OFF")
        .define("ENABLE_GLIB", "OFF")
        .define("ENABLE_GOBJECT_INTROSPECTION", "OFF")
        .define("ENABLE_CPP", "OFF")
        .define("ENABLE_GTK_DOC", "OFF")
        .define("ENABLE_QT5", "OFF")
        .define("ENABLE_QT6", "OFF")
        .define("ENABLE_LIBOPENJPEG", "unmaintained")
        .define("ENABLE_CMS", "none")
        .define("ENABLE_LIBCURL", "OFF")
        .define("ENABLE_ZLIB", "OFF")
        .define("ENABLE_DCTDECODER", "unmaintained")
        .define("ENABLE_BOOST", "OFF")
        .define("ENABLE_ZLIB_UNCOMPRESS", "OFF")
        .define("SPLASH_CMYK", "OFF")
        .define("WITH_JPEG", "OFF")
        .define("WITH_PNG", "OFF")
        .define("WITH_TIFF", "OFF")
        .define("WITH_NSS3", "OFF")
        .define("WITH_Cairo", "OFF")
        .define("WITH_FONTCONFIGURATION_FONTCONFIG", "OFF")
        .define("RUN_GPERF_IF_PRESENT", "OFF")
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build",
        static_build.display()
    );
    println!("cargo:rustc-link-lib=static=poppler");

    let dir = std::path::Path::new(&static_build).join("include/poppler");

    let dir_config = std::path::Path::new(&static_build).join("build/poppler");
    let callpoppler = callpoppler
        .include(dir)
        .include(dir_config)
        .flag_if_supported("-std=c++20");

    callpoppler
}

// #[cfg(any(not(feature = "static-poppler"), not(feature = "static-poppler-wasm")))]
#[cfg(not(feature = "static-poppler-wasm"))]
fn setup(mut callpoppler: &mut cc::Build) -> &mut cc::Build {
    let poppler = pkg_config::Config::new()
        .cargo_metadata(true)
        .probe("poppler")
        .expect("pkg-config could not find poppler");

    for dir in &poppler.include_paths {
        callpoppler = callpoppler.include(dir);
    }
    callpoppler.flag_if_supported("-std=c++20");

    callpoppler
}

fn main() {
    if let Ok(_) = std::env::var("DOCS_RS") {
        return;
    }

    let mut build = cc::Build::new();

    let callpoppler = build.cpp(true).warnings(false).file("src/callpoppler.cc");

    let callpoppler = setup(callpoppler);

    callpoppler.compile("callpoppler.a");

    //shouldn't cc take care of this?
    println!("cargo:rerun-if-changed=src/callpoppler.cc");
}
