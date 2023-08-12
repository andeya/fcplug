#![feature(result_option_inspect)]
#![feature(try_trait_v2)]

use std::process::Command;
use std::process::Output as CmdOutput;

use crate::ffidl::FFIDL;

pub use self::ffidl::{Config, GoObjectPath, UnitLikeStructPath};

#[allow(dead_code)]
enum GenMode {
    Codec,
    NoCodec,
}

#[cfg(feature = "no-codec")]
const GEN_MODE: GenMode = GenMode::NoCodec;
#[cfg(not(feature = "no-codec"))]
const GEN_MODE: GenMode = GenMode::Codec;

#[cfg(not(debug_assertions))]
const BUILD_MODE: &'static str = "release";
#[cfg(debug_assertions)]
const BUILD_MODE: &'static str = "debug";

mod ffidl;
mod os_arch;
mod go_os_arch_gen;
mod rust_os_arch_gen;

pub fn generate_code(config: Config)  {
    FFIDL::generate(config)
}

fn exit_with_warning(code: i32, message: impl AsRef<str>) {
    println!("cargo:warning={}", message.as_ref());
    std::process::exit(code);
}

fn new_shell_cmd() -> Command {
    let mut param = ("sh", "-c");
    if cfg!(target_os = "windows") {
        param.0 = "cmd";
        param.1 = "/c";
    }
    let mut cmd = Command::new(param.0);
    cmd.arg(param.1);
    cmd
}


fn deal_output(output: CmdOutput) {
    if !output.status.success() {
        eprintln!("{output:?}");
        exit_with_warning(output.status.code().unwrap_or(-1), format!("{output:?}"));
    } else {
        if output.stderr.is_empty() {
            println!("{output:?}");
        } else {
            println!("cargo:warning={:?}", String::from_utf8(output.stderr.clone()).unwrap_or(format!("{output:?}")));
        }
    }
}
