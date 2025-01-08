use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum BuildModel {
    Debug,
    Release,
}

pub const fn build_channel() -> BuildModel {
    if cfg!(debug_assertions) {
        return BuildModel::Debug;
    }
    BuildModel::Release
}

impl Display for BuildModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Debug => f.write_str("debug"),
            Self::Release => f.write_str("release"),
        }
    }
}

pub const fn is_debug() -> bool {
    matches!(build_channel(), BuildModel::Debug)
}

pub const fn is_release() -> bool {
    matches!(build_channel(), BuildModel::Release)
}

#[cfg(test)]
mod tests {
    use xshell::{cmd, Shell};

    #[test]
    fn test_std() {
        let sh = Shell::new().unwrap();
        let repo = "is_debug_test";
        let _ = cmd!(sh, "rm -rf {repo}").run();
        cmd!(sh, "cargo new {repo}").run().unwrap();
        sh.change_dir(repo);
        let cargo_content = r#"
        [package]
name = "is_debug_test"
version = "0.1.0"
edition = "2021"

[dependencies]
is_debug = {path = "../"}
        "#;

        sh.write_file("Cargo.toml", cargo_content).unwrap();

        let main_debug = r#"
fn main() {
    assert!(is_debug::is_debug())
}
"#;

        let main_release = r#"
fn main() {
    assert!(is_debug::is_release())
}
"#;
        cmd!(sh, "pwd").run().unwrap();
        sh.write_file("src/main.rs", main_debug).unwrap();
        cmd!(sh, "cargo run").run().unwrap();
        sh.write_file("src/main.rs", main_release).unwrap();
        cmd!(sh, "cargo run --release").run().unwrap();

        cmd!(sh, "rm -rf ../{repo}").run().unwrap();
    }
}
