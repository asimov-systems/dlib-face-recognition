fn main() {
    let mut config = cpp_build::Config::new();

    #[cfg(target_family = "windows")]
    {
        let dlib = vcpkg::find_package("dlib").unwrap();

        for include in dlib.include_paths {
            config.include(include);
        }
    }

    #[cfg(not(target_family = "windows"))]
    {
        let dlib = pkg_config::Config::new()
            .statik(true)
            .probe("dlib")
            .unwrap();

        for include in dlib.include_paths {
            config.include(include);
        }
    }

    config.build("src/lib.rs");
}
