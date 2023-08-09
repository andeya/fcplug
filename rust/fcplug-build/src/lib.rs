#![feature(result_option_inspect)]
#![feature(try_trait_v2)]

use crate::ffidl::FFIDL;

pub use self::ffidl::{Config, GoObjectPath, UnitLikeStructPath};

mod ffidl;
mod os_arch;
mod go_os_arch_gen;
mod rust_os_arch_gen;

pub fn generate_code(config: Config) -> anyhow::Result<()> {
    FFIDL::generate(config)
}
