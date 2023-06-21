use std::env;
use std::path::{Component, MAIN_SEPARATOR_STR, PathBuf};

use crate::Report;

pub fn gen_rust_callee_code() -> Report {
    let base = target_profile_dir().as_os_str()
        .to_str()
        .unwrap()
        .to_string();
    let cargo_pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let report = Report {
        rust_c_header_filename: base.clone() + MAIN_SEPARATOR_STR + &cargo_pkg_name.replace("-", "_") + ".h",
        rust_c_lib_filename: base + MAIN_SEPARATOR_STR + "lib" + &cargo_pkg_name.replace("-", "_") + ".a",
    };
    println!("build-log: {:?}", report);
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_parse_expand(&[cargo_pkg_name.as_str()])
        .with_after_include(if cargo_pkg_name != "fcplug-callee" {
            r#"
typedef enum OriginType {
  Vec = 0,
  FlatBuffer = 1,
} OriginType;

typedef enum ResultCode {
  NoError = 0,
  Decode = 1,
  Encode = 2,
} ResultCode;

typedef struct Buffer {
  uint8_t *ptr;
  uintptr_t len;
  uintptr_t cap;
} Buffer;

typedef struct LeakBuffer {
  enum OriginType free_type;
  uintptr_t free_ptr;
  struct Buffer buffer;
} LeakBuffer;

typedef struct FFIResult {
  enum ResultCode code;
  struct LeakBuffer data;
} FFIResult;

void free_buffer(enum OriginType free_type, uintptr_t free_ptr);
"#
        } else { "" })
        .generate()
        .expect("Unable to generate C header file")
        .write_to_file(&report.rust_c_header_filename);
    report
}

fn target_profile_dir() -> PathBuf {
    let mut p = PathBuf::new();
    PathBuf::from(&env::var("OUT_DIR").unwrap())
        .components()
        .rev()
        .skip(3)
        .collect::<Vec<Component>>()
        .into_iter()
        .rev()
        .for_each(|c| p.push(c.as_os_str()));
    p
}
