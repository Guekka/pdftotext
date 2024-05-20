fn setup(mut callpoppler: &mut cc::Build) -> &mut cc::Build {
    let poppler = pkg_config::Config::new()
        .cargo_metadata(true)
        .probe("poppler")
        .expect("pkg-config could not find poppler");

    for dir in &poppler.include_paths {
        callpoppler = callpoppler.include(dir);
    }
    // gcc and similar
    callpoppler.flag_if_supported("-std=c++20");
    // msvc
    callpoppler.flag_if_supported("/std:c++20");

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
