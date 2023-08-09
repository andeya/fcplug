#![allow(non_camel_case_types)]

#[derive(strum::AsRefStr, strum::EnumString)]
pub enum GoOS {
    aix,
    android,
    darwin,
    dragonfly,
    freebsd,
    illumos,
    ios,
    js,
    linux,
    netbsd,
    openbsd,
    plan9,
    solaris,
    windows,
}

#[derive(strum::AsRefStr, strum::EnumString)]
pub enum GoArch {
    #[strum(serialize = "386")]_386,
    amd64,
    arm,
    arm64,
    loong64,
    mips,
    mips64,
    mips64le,
    mipsle,
    ppc64,
    ppc64le,
    riscv64,
    s390x,
    wasm,
}
