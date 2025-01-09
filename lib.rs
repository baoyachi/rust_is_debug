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
        sh.write_file("src/main.rs", main_debug).unwrap();
        cmd!(sh, "cargo run").run().unwrap();
        sh.write_file("src/main.rs", main_release).unwrap();
        cmd!(sh, "cargo run --release").run().unwrap();

        cmd!(sh, "rm -rf ../{repo}").run().unwrap();
    }

    #[test]
    #[cfg(feature = "no_std")]
    fn test_no_std() {
        let sh = Shell::new().unwrap();
        let repo = "is_debug_no_std";
        let _ = cmd!(sh, "rm -rf {repo}").run();
        cmd!(sh, "cargo new {repo}").run().unwrap();
        sh.change_dir(repo);

        let cargo_config = r#"
[target.riscv32imc-unknown-none-elf]
runner = "espflash flash --monitor"

[build]
rustflags = [
  "-C", "link-arg=-Tlinkall.x",
  # Required to obtain backtraces (e.g. when using the "esp-backtrace" crate.)
  # NOTE: May negatively impact performance of produced code
  "-C", "force-frame-pointers",
]

target = "riscv32imc-unknown-none-elf"

[unstable]
build-std = ["core"]

[env]
ESP_LOG="INFO"
"#;
        sh.write_file(".cargo/config.toml", cargo_config).unwrap();

        let cargo_dep_content = r#"
[package]
name = "is_debug_no_std"
version = "0.1.0"
edition = "2021"

[dependencies]
esp-backtrace = { version = "0.14.2", features = [
    "esp32c3",
    "exception-handler",
    "panic-handler",
    "println",
]}

esp-hal = { version = "0.22.0", features = [
    "esp32c3",
] }
esp-println = { version = "0.12.0", features = ["esp32c3", "log"] }
log = { version = "0.4.22" }

# is_debug = {path = "../"}
"#;

        sh.write_file("Cargo.toml", cargo_dep_content).unwrap();

        let main_content = r#"
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::prelude::*;
use log::info;

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();
    loop {
        info!("Hello world!");
        delay.delay(500.millis());
    }
}
"#;

        let rust_toolchain = r#"
[toolchain]
channel = "stable"
components = ["rust-src"]
targets = ["riscv32imc-unknown-none-elf"]"#;

        sh.write_file("src/main.rs", main_content).unwrap();
        sh.write_file("rust-toolchain.toml", rust_toolchain)
            .unwrap();
        cmd!(sh, "cargo build --release").run().unwrap();

        cmd!(sh, "rm -rf ../{repo}").run().unwrap();
    }
}
