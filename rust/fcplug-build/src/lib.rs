#![feature(result_option_inspect)]
#![feature(try_trait_v2)]
#![allow(dead_code)]

use std::fmt::Debug;
use std::io;
use std::process::Command;
use std::process::Output as CmdOutput;

use backtrace::Backtrace;

pub use config::{Config, GoObjectPath, UnitLikeStructPath};

use crate::generator::Generator;

mod config;
mod os_arch;
mod go_os_arch_gen;
mod rust_os_arch_gen;
mod generator;
#[cfg(feature = "no-codec")]
mod without_codec;
#[cfg(not(feature = "no-codec"))]
mod with_codec;


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

pub fn generate_code(config: Config) {
    Generator::generate(config)
}

const CODE_UNKNOWN: i32 = -1;
const CODE_CMD_UNKNOWN: i32 = -2;
const CODE_IO: i32 = -3;

fn exit_with_warning(code: i32, message: impl AsRef<str>) {
    let mut frames = vec![];
    for bf in Backtrace::new().frames()[4..].as_ref() {
        if bf.symbols().get(0).unwrap().name().unwrap().to_string().starts_with("build_script_build::main::") {
            break;
        }
        frames.push(bf.clone());
    }
    println!("cargo:warning={}\nbacktrace:\n{:?}", message.as_ref(), Backtrace::from(frames));
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

fn deal_result<T, E: Debug>(code: i32, result: Result<T, E>) -> T {
    match result {
        Ok(t) => { t }
        Err(e) => {
            exit_with_warning(code, format!("{e:?}"));
            unreachable!()
        }
    }
}

fn deal_output(output: io::Result<CmdOutput>) {
    match output {
        Ok(output) => {
            if !output.status.success() {
                exit_with_warning(output.status.code().unwrap_or(CODE_CMD_UNKNOWN), format!("{output:?}"));
            } else {
                if output.stderr.is_empty() {
                    println!("{output:?}");
                } else {
                    println!("cargo:warning={:?}", String::from_utf8(output.stderr.clone()).unwrap_or(format!("{output:?}")));
                }
            }
        }
        Err(e) => {
            exit_with_warning(CODE_UNKNOWN, format!("{e:?}"));
        }
    }
}
