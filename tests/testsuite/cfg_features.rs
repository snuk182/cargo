use support::project;

#[test]
fn syntax() {
    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "a"
            version = "0.0.1"
            authors = []

            [target.'cfg(unix)'.features]
            b = []
            [target.'cfg(windows)'.features]
            b = []
        "#,
        ).file("src/lib.rs", r#"
            pub fn bb() {}
        "#).build();
    p.cargo("build -v").run();
}

#[test]
fn include_by_param() {
    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "a"
            version = "0.0.1"
            authors = []

            [target.'cfg(unix)'.features]
            b = []
            [target.'cfg(windows)'.features]
            c = []
        "#,
        ).file("src/lib.rs", r#"
            #[cfg(feature = "b")]
            pub const BB: usize = 0;
            #[cfg(feature = "c")]
            pub const BB: usize = 1;
            
            pub fn bb() -> Result<(), ()> { if BB > 0 { Ok(()) } else { Err(()) } }
        "#).build();
    p.cargo(format!("build -v --features {}", if cfg!(unix) { "b" } else { "c" }).as_str()).run();
}

#[test]
fn dont_include_by_platform() {
    let other_family = if cfg!(unix) { "windows" } else { "unix" };
    let p = project()
        .file(
            "Cargo.toml",
            &format!(
                r#"
            [package]
            name = "a"
            version = "0.0.1"
            authors = []

            [target.'cfg({})'.features]
            b = []
        "#,
                other_family
            ),
        ).file("src/lib.rs", r#"
            #[cfg(feature = "b")]
            pub const BB: usize = 0;
            
            pub fn bb() { let _ = BB; }
        "#).build();
    p.cargo("build --features b -vv")
        .with_status(101)
        .with_stderr_contains(
            "\
error[E0425]: cannot find value `BB` in this scope",
        ).run();
}

#[test]
fn dont_include_by_param() {
    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "a"
            version = "0.0.1"
            authors = []

            [target.'cfg(unix)'.features]
            b = []
            [target.'cfg(windows)'.features]
            c = []
        "#,
        ).file("src/lib.rs", r#"
            #[cfg(feature = "b")]
            pub const BB: usize = 0;
            #[cfg(feature = "c")]
            pub const BB: usize = 1;
            
            pub fn bb() -> Result<(), ()> { if BB > 0 { Ok(()) } else { Err(()) } }
        "#).build();
    p.cargo("build -v")
        .with_status(101)
        .with_stderr_contains(
            "\
error[E0425]: cannot find value `BB` in this scope",
        ).run();
}

#[test]
fn dont_include_default() {
    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "a"
            version = "0.0.1"
            authors = []

            [target.'cfg(unix)'.features]
            b = []
            
            [features]
            default = ["b"]
        "#,
        ).file("src/lib.rs", r#"
            #[cfg(feature = "b")]
            pub const BB: usize = 0;
            
            pub fn bb() { let _ = BB; }
        "#).build();
    p.cargo("build -v")
        .with_status(101)
        .with_stderr_contains(
            "\
error[E0425]: cannot find value `BB` in this scope",
        ).run();
}

// https://github.com/rust-lang/cargo/issues/5313
#[test]
#[cfg(all(
    target_arch = "x86_64",
    target_os = "linux",
    target_env = "gnu"
))]
fn cfg_looks_at_rustflags_for_target() {
    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "a"
            version = "0.0.1"
            authors = []

            [target.'cfg(with_b)'.features]
            b = []
        "#,
        ).file(
            "src/main.rs",
            r#"
            #[cfg(with_b)]
            pub const BB: usize = 0;

            fn main() { let _ = BB; }
        "#,
        ).build();

    p.cargo("build --target x86_64-unknown-linux-gnu")
        .env("RUSTFLAGS", "--cfg with_b")
        .run();
}
