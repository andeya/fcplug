use crate::ffidl::FFIDL;

pub use self::ffidl::{Config, GoObjectPath, UnitLikeStructPath};

mod ffidl;

pub fn generate_code(config: Config) -> anyhow::Result<()> {
    FFIDL::generate(config)
}
