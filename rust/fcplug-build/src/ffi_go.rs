use std::{env, fs, str};
use std::path::PathBuf;
use std::process::Command;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{BuildConfig, Report};

pub(crate) const FILE_NAME: &'static str = "ffi.c.go";

pub(crate) const FILE_TPL: &'static str = r##########"// Code generated by fcplug. DO NOT EDIT.

package ${package}

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: -L. -l${c_header_name_base}

#include "${c_header_name_base}.h"
*/
import "C"
import (
	"reflect"
	"unsafe"

	"github.com/andeya/fcplug/go/caller"
)

func bytesToBuffer(b []byte) C.struct_Buffer {
	return C.struct_Buffer{
		ptr: (*C.uint8_t)(unsafe.Pointer(&b[0])),
		len: C.uintptr_t(len(b)),
		cap: C.uintptr_t(cap(b)),
	}
}

func bufferToBytes(buf C.struct_Buffer) []byte {
	return *(*[]byte)(unsafe.Pointer(&reflect.SliceHeader{
		Data: uintptr(unsafe.Pointer(buf.ptr)),
		Len:  int(buf.len),
		Cap:  int(buf.cap),
	}))
}

func toResultCode(code C.enum_ResultCode) caller.ResultCode {
	switch code {
	case C.NoError:
		return caller.CodeNoError
	case C.Decode:
		return caller.CodeDecode
	case C.Encode:
		return caller.CodeEncode
	default:
		return caller.CodeUnknown
	}
}

${fn_list}
"##########;

pub(crate) const RAW_FN_TPL: &'static str = r##########"
//go:inline
func C_${c_fn_name}(req []byte) (res caller.ABIResult[[]byte], free func()) {
	r := C.${c_fn_name}(bytesToBuffer(req))
	if code := toResultCode(r.code); code != caller.CodeNoError {
		return caller.ABIResult[[]byte]{
			Code: code,
		}, func() {}
	}
	return caller.ABIResult[[]byte]{
		Data: bufferToBytes(r.data.buffer),
	}, func() { C.free_buffer(r.data.free_type, r.data.free_ptr) }
}
"##########;

pub(crate) const PB_FN_TPL: &'static str = r##########"
//go:inline
func C_${c_fn_name}_bytes(req []byte) (res caller.ABIResult[[]byte], free func()) {
	r := C.${c_fn_name}(bytesToBuffer(req))
	if code := toResultCode(r.code); code != caller.CodeNoError {
		return caller.ABIResult[[]byte]{
			Code: code,
		}, func() {}
	}
	return caller.ABIResult[[]byte]{
		Data: bufferToBytes(r.data.buffer),
	}, func() { C.free_buffer(r.data.free_type, r.data.free_ptr) }
}

//go:inline
func C_${c_fn_name}[T caller.PbMessage](req caller.PbMessage) caller.ABIResult[T] {
	b, code := caller.PbMarshal(req)
	if code.IsErr() {
		return caller.ABIResult[T]{Code: code}
	}
	r := C.${c_fn_name}(bytesToBuffer(b))
	if code := toResultCode(r.code); code != caller.CodeNoError {
		return caller.ABIResult[T]{
			Code: code,
		}
	}
	defer C.free_buffer(r.data.free_type, r.data.free_ptr)
	return caller.PbUnmarshal[T](bufferToBytes(r.data.buffer))
}
"##########;

pub(crate) const FB_FN_TPL: &'static str = r##########"
//go:inline
func C_${c_fn_name}_bytes(req []byte) (resp caller.ABIResult[[]byte], free func()) {
	resp = caller.ABIResult[[]byte]{}
	r := C.${c_fn_name}(bytesToBuffer(req))
	resp.Code = toResultCode(r.code)
	if resp.Code != caller.CodeNoError {
		return resp, func() {}
	}
	resp.Data = bufferToBytes(r.data.buffer)
	return resp, func() {
		C.free_buffer(r.data.free_type, r.data.free_ptr)
	}
}
"##########;

pub struct PbGoConfig {
    pub filename: String,
}

pub(crate) fn gen_go_code(config: &BuildConfig, report: &Report) {
    if env::var("CARGO_PKG_NAME").unwrap() == "fcplug-callee" {
        return;
    }

    let go_out_dir = config.go_out_dir.to_str().unwrap();

    // protobuf code
    if let Some(pb_go_config) = &config.pb_go_config {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/c")
                .arg(format!(
                    "protoc --proto_path={} --go_out {} {}",
                    PathBuf::from(&pb_go_config.filename)
                        .parent()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                    go_out_dir,
                    pb_go_config.filename,
                ))
                .output()
                .unwrap();
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "protoc --proto_path={} --go_out {} {}",
                    PathBuf::from(&pb_go_config.filename)
                        .parent()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                    go_out_dir,
                    pb_go_config.filename,
                ))
                .output()
                .unwrap();
        }
    }

    // caller code

    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"FFIResult (?P<c_fn_name>[A-Z_a-z0-9]+)\(Buffer req\);").unwrap();
    }
    let header = fs::read(&report.c_header_filename);
    if header.is_err() {
        println!("{}", header.err().unwrap());
        return;
    }
    let header = header.unwrap();
    let fn_list = RE
        .captures_iter(str::from_utf8(&header).unwrap())
        .map(|cap| cap["c_fn_name"].to_string())
        .collect::<Vec<String>>();

    println!("fn_list: {:?}", fn_list);

    let fn_list = fn_list
        .iter()
        .map(|c_fn_name| {
            if c_fn_name.starts_with("ffi_raw_") {
                RAW_FN_TPL.replace("${c_fn_name}", c_fn_name)
            } else if c_fn_name.starts_with("ffi_pb_") {
                PB_FN_TPL.replace("${c_fn_name}", c_fn_name)
            } else if c_fn_name.starts_with("ffi_fb_") {
                FB_FN_TPL.replace("${c_fn_name}", c_fn_name)
            } else {
                String::new()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    let file_txt = FILE_TPL
        .replace(
            "${package}",
            PathBuf::from(&go_out_dir)
                .canonicalize()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        )
        .replace(
            "${c_header_name_base}",
            PathBuf::from(&report.c_header_filename)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .trim_end_matches(".h"),
        )
        .replace("${fn_list}", &fn_list);

    fs::write(config.go_out_dir.join(&FILE_NAME), file_txt.as_bytes()).unwrap();
}
