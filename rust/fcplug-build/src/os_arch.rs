use std::env;
use std::str::FromStr;

pub(crate) use crate::go_os_arch_gen::{GoArch, GoOS};
pub(crate) use crate::rust_os_arch_gen::{RustArch, RustOS};

pub(crate) fn parse_target_triple(target_triple: &str) -> Result<(RustOS, RustArch), String> {
    let a = target_triple
        .replace(".", "_")
        .split("-")
        .map(ToOwned::to_owned)
        .collect::<Vec<String>>();
    Ok((
        RustOS::from_str(a.get(2).unwrap_or(&"".to_string())).map_err(|_| { format!("unknown os {:?}", a.get(2)) })?,
        RustArch::from_str(a.get(0).unwrap_or(&"".to_string())).map_err(|_| { format!("unknown arch {:?}", a.get(0)) })?,
    ))
}

pub(crate) fn parse_target_triple_from_env() -> Result<(RustOS, RustArch), String> {
    parse_target_triple(env::var("TARGET").unwrap().as_str())
}

pub(crate) fn get_go_os_arch_from_env() -> Result<(GoOS, GoArch), String> {
    let (os, arch) = parse_target_triple_from_env()?;
    Ok((GoOS::try_from(os)?, GoArch::try_from(arch)?))
}

impl TryFrom<RustArch> for GoArch {
    type Error = String;

    fn try_from(value: RustArch) -> Result<Self, Self::Error> {
        match value {
            RustArch::aarch64 => Ok(GoArch::amd64),
            RustArch::aarch64_be => Ok(GoArch::amd64),
            RustArch::arm => Ok(GoArch::arm),
            RustArch::arm64_32 => Ok(GoArch::arm64),
            RustArch::armeb => Ok(GoArch::arm),
            RustArch::armebv7r => Ok(GoArch::arm),
            RustArch::armv4t => Ok(GoArch::arm),
            RustArch::armv5te => Ok(GoArch::arm),
            RustArch::armv6 => Ok(GoArch::arm),
            RustArch::armv6k => Ok(GoArch::arm),
            RustArch::armv7 => Ok(GoArch::arm),
            RustArch::armv7a => Ok(GoArch::arm),
            RustArch::armv7k => Ok(GoArch::arm),
            RustArch::armv7r => Ok(GoArch::arm),
            RustArch::armv7s => Ok(GoArch::arm),
            RustArch::asmjs => Ok(GoArch::wasm),
            RustArch::avr => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::bpfeb => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::bpfel => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::hexagon => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::i386 => Ok(GoArch::_386),
            RustArch::i586 => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::i686 => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::loongarch64 => Ok(GoArch::loong64),
            RustArch::m68k => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::mips => Ok(GoArch::mips),
            RustArch::mips64 => Ok(GoArch::mips64),
            RustArch::mips64el => Ok(GoArch::mips64le),
            RustArch::mipsel => Ok(GoArch::mipsle),
            RustArch::mipsisa32r6 => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::mipsisa32r6el => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::mipsisa64r6 => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::mipsisa64r6el => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::msp430 => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::nvptx64 => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::powerpc => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::powerpc64 => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::powerpc64le => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::riscv32gc => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::riscv32i => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::riscv32im => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::riscv32imac => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::riscv32imc => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::riscv64gc => Ok(GoArch::riscv64),
            RustArch::riscv64imac => Ok(GoArch::riscv64),
            RustArch::s390x => Ok(GoArch::s390x),
            RustArch::sparc => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::sparc64 => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::sparcv9 => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::thumbv4t => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::thumbv5te => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::thumbv6m => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::thumbv7a => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::thumbv7em => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::thumbv7m => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::thumbv7neon => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::thumbv8m_base => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::thumbv8m_main => Err(format!("{} not supported by golang", value.as_ref())),
            RustArch::wasm32 => Ok(GoArch::wasm),
            RustArch::wasm64 => Ok(GoArch::wasm),
            RustArch::x86_64 => Ok(GoArch::amd64),
            RustArch::x86_64h => Ok(GoArch::amd64),
        }
    }
}

impl TryFrom<RustOS> for GoOS {
    type Error = String;

    fn try_from(value: RustOS) -> Result<Self, Self::Error> {
        match value {
            RustOS::_3ds => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::aix => Ok(GoOS::aix),
            RustOS::android => Ok(GoOS::android),
            RustOS::androideabi => Ok(GoOS::android),
            RustOS::cuda => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::darwin => Ok(GoOS::darwin),
            RustOS::dragonfly => Ok(GoOS::dragonfly),
            RustOS::eabi => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::eabihf => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::elf => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::emscripten => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::espidf => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::freebsd => Ok(GoOS::freebsd),
            RustOS::fuchsia => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::gnu => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::haiku => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::hermit => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::illumos => Ok(GoOS::illumos),
            RustOS::ios => Ok(GoOS::ios),
            RustOS::l4re => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::linux => Ok(GoOS::linux),
            RustOS::netbsd => Ok(GoOS::netbsd),
            RustOS::none => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::nto => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::openbsd => Ok(GoOS::openbsd),
            RustOS::psp => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::psx => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::redox => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::solaris => Ok(GoOS::solaris),
            RustOS::solid_asp3 => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::switch => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::tvos => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::uefi => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::unknown => Ok(GoOS::js),
            RustOS::vita => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::vxworks => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::watchos => Err(format!("{} not supported by golang", value.as_ref())),
            RustOS::windows => Ok(GoOS::windows),
            RustOS::xous => Err(format!("{} not supported by golang", value.as_ref())),
        }
    }
}
