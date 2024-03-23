use std::sync::Arc;

use pilota_build::rir::Method;
use pilota_build::ty::TyKind;
use pilota_build::{rir::Service, DefId};

use crate::generator::{GoCodegenBackend, GoGeneratorBackend};

impl GoCodegenBackend for GoGeneratorBackend {
    // {lib}.go
    fn codegen_rustffi_iface_method(
        &self,
        service_def_id: DefId,
        method: &Arc<Method>,
    ) -> Option<(String, String)> {
        let iface_method_name = self.iface_method_name(method);
        let args_sign = method
            .args
            .iter()
            .map(|arg| {
                if arg.ty.is_scalar() {
                    format!("{} {}", self.arg_name(arg), self.arg_type(arg, false))
                } else {
                    format!(
                        "{} TBytes[*{}]",
                        self.arg_name(arg),
                        self.arg_type(arg, false)
                    )
                }
            })
            .collect::<Vec<String>>()
            .join(",");
        let ret_type = self.ret_type(method, false);
        let iface_method = format!("{iface_method_name}({args_sign}) RustFfiResult[{ret_type}]");
        let ffi_func_name = self.ffi_func_name(service_def_id, method);
        let args_assign = method
            .args
            .iter()
            .map(|arg| {
                if arg.ty.is_scalar() {
                    let name = self.arg_name(arg);
                    if let TyKind::Bool = arg.ty.kind {
                        format!("C._Bool({name})")
                    } else {
                        name
                    }
                } else {
                    format!("{}.asBuffer()", self.arg_name(arg))
                }
            })
            .collect::<Vec<String>>()
            .join(",");
        Some((
            iface_method,
            format!("return newRustFfiResult[{ret_type}](C.{ffi_func_name}({args_assign}))"),
        ))
    }
    // {lib}.go
    fn codegen_rustffi_service_impl(&self, _service_def_id: DefId, _s: &Service) -> String {
        r###"
type ResultCode = int8

const (
	RcNoError ResultCode = 0
	RcDecode  ResultCode = -1
	RcEncode  ResultCode = -2
	RcUnknown ResultCode = -128
)

// TBytes bytes with type marker
type TBytes[T any] struct {
	bytes []byte
	_nil  *T
}

// TBytesFromBytes new TBytes from bytes
//go:inline
func TBytesFromBytes[T any](bytes []byte) TBytes[T] {
	return TBytes[T]{bytes: bytes}
}

// TBytesFromString new TBytes from string
//go:inline
func TBytesFromString[T any](s string) TBytes[T] {
	return TBytes[T]{bytes: valconv.StringToReadonlyBytes[string](s)}
}

//go:inline
func TBytesFromPbUnchecked[T proto.Message](obj T) TBytes[T] {
	tb, _ := TBytesFromPb[T](obj)
	return tb
}

//go:inline
func TBytesFromPb[T proto.Message](obj T) (TBytes[T], error) {
	var tb TBytes[T]
	var err error
	tb.bytes, err = proto.Marshal(obj)
	if err != nil {
		return TBytes[T]{}, err
	}
	return tb, nil
}

//go:inline
func TBytesFromJsonUnchecked[T proto.Message](obj T) TBytes[T] {
	tb, _ := TBytesFromJson[T](obj)
	return tb
}

//go:inline
func TBytesFromJson[T any](obj T) (TBytes[T], error) {
	var tb TBytes[T]
	var err error
	tb.bytes, err = sonic.Marshal(obj)
	if err != nil {
		return TBytes[T]{}, err
	}
	return tb, nil
}

//go:inline
func (b TBytes[T]) Len() int {
	return len(b.bytes)
}

// PbUnmarshal as protobuf to unmarshal
// NOTE: maybe reference Rust memory buffer
//
//go:inline
func (b TBytes[T]) PbUnmarshal() (*T, error) {
	var t T
	if b.Len() > 0 {
		err := proto.Unmarshal(b.bytes, any(&t).(proto.Message))
		if err != nil {
			return nil, err
		}
	}
	return &t, nil
}

// PbUnmarshalUnchecked as protobuf to unmarshal
// NOTE: maybe reference Rust memory buffer
//
//go:inline
func (b TBytes[T]) PbUnmarshalUnchecked() (*T) {
	var t T
	if b.Len() > 0 {
		_= proto.Unmarshal(b.bytes, any(&t).(proto.Message))
	}
	return &t
}

// JsonUnmarshal as json to unmarshal
// NOTE: maybe reference Rust memory buffer
//
//go:inline
func (b TBytes[T]) JsonUnmarshal() (*T, error) {
	var t T
	if b.Len() > 0 {
		err := sonic.Unmarshal(b.bytes, &t)
		if err != nil {
			return nil, err
		}
	}
	return &t, nil
}

// JsonUnmarshalUnchecked as json to unmarshal
// NOTE: maybe reference Rust memory buffer
//
//go:inline
func (b TBytes[T]) JsonUnmarshalUnchecked() *T {
	var t T
	if b.Len() > 0 {
		_ = sonic.Unmarshal(b.bytes, &t)
	}
	return &t
}

// Unmarshal unmarshal to object
// NOTE: maybe reference Rust memory buffer
//
//go:inline
func (b TBytes[T]) Unmarshal(unmarshal func([]byte, any) error) (*T, error) {
	var t T
	if b.Len() > 0 {
		err := unmarshal(b.bytes, &t)
		if err != nil {
			return nil, err
		}
	}
	return &t, nil
}

// UnmarshalUnchecked unmarshal to object
// NOTE: maybe reference Rust memory buffer
//
//go:inline
func (b TBytes[T]) UnmarshalUnchecked(unmarshal func([]byte, any) error) *T {
	var t T
	if b.Len() > 0 {
		_ = unmarshal(b.bytes, &t)
	}
	return &t
}

//go:inline
func (b TBytes[T]) ForCBuffer() (unsafe.Pointer, int) {
	size := len(b.bytes)
	if size == 0 {
		return nil, 0
	}
	if cap(b.bytes) > size {
		b.bytes = b.bytes[0:size:size]
	}
	return unsafe.Pointer(&b.bytes[0]), size
}

//go:inline
func (b TBytes[T]) asBuffer() C.struct_Buffer {
	p, size := b.ForCBuffer()
	if size == 0 {
		return C.struct_Buffer{}
	}
	return C.struct_Buffer{
		ptr: (*C.uint8_t)(p),
		len: C.uintptr_t(size),
		cap: C.uintptr_t(size),
	}
}

// CBuffer Rust buffer for Go
type CBuffer struct {
	buf C.struct_Buffer
}

// Free free rust memory buffer, must be called!
//
//go:inline
func (b CBuffer) Free() {
	if b.buf.len > 0 {
		C.free_buffer(b.buf)
	}
}

//go:inline
func (b CBuffer) Len() int {
	return int(b.buf.len)
}

//go:inline
func (b CBuffer) AsBytes() []byte {
	if b.buf.len == 0 {
		return nil
	}
	return *(*[]byte)(unsafe.Pointer(&reflect.SliceHeader{
		Data: uintptr(unsafe.Pointer(b.buf.ptr)),
		Len:  int(b.buf.len),
		Cap:  int(b.buf.cap),
	}))
}

//go:inline
func (b CBuffer) AsString() string {
    if b.buf.len == 0 {
		return ""
	}
	return valconv.BytesToString[string](b.AsBytes())
}

// RustFfiResult Rust FFI Result for Go
// NOTE: must call Free method to free rust memory buffer!
type RustFfiResult[T any] struct {
	CBuffer
	Code ResultCode
	_nil *T
}

//go:inline
func newRustFfiResult[T any](ret C.struct_RustFfiResult) RustFfiResult[T] {
	return RustFfiResult[T]{
		CBuffer: CBuffer{buf: ret.data},
		Code:    ResultCode(ret.code),
		_nil:    nil,
	}
}

//go:inline
func (r RustFfiResult[T]) String() string {
	return fmt.Sprintf("Code: %d, CBuffer: %s", r.Code, r.CBuffer.AsString())
}

//go:inline
func (r RustFfiResult[T]) IsOk() bool {
	return r.Code == RcNoError
}

// AsError as an error
// NOTE: reference Rust memory buffer
//
//go:inline
func (r RustFfiResult[T]) AsError() error {
	if r.Code != RcNoError {
		return errors.New(r.AsString())
	}
	return nil
}

// PbUnmarshal as protobuf to unmarshal
// NOTE: maybe reference Rust memory buffer
//
//go:inline
func (r RustFfiResult[T]) PbUnmarshal() (*T, error) {
	if err := r.AsError(); err != nil {
		return nil, err
	}
	var t T
	if r.Len() > 0 {
		err := proto.Unmarshal(r.AsBytes(), any(&t).(proto.Message))
		if err != nil {
			return nil, err
		}
	}
	return &t, nil
}

// PbUnmarshalUnchecked as protobuf to unmarshal
// NOTE: maybe reference Rust memory buffer
//
//go:inline
func (r RustFfiResult[T]) PbUnmarshalUnchecked() *T {
	if err := r.AsError(); err != nil {
		return nil
	}
	var t T
	if r.Len() > 0 {
		_ = proto.Unmarshal(r.AsBytes(), any(&t).(proto.Message))
	}
	return &t
}

// JsonUnmarshal as json to unmarshal
// NOTE: maybe reference Rust memory buffer
//
//go:inline
func (r RustFfiResult[T]) JsonUnmarshal() (*T, error) {
	if err := r.AsError(); err != nil {
		return nil, err
	}
	var t T
	if r.Len() > 0 {
		err := sonic.Unmarshal(r.AsBytes(), &t)
		if err != nil {
			return nil, err
		}
	}
	return &t, nil
}

// JsonUnmarshalUnchecked as json to unmarshal
// NOTE: maybe reference Rust memory buffer
//
//go:inline
func (r RustFfiResult[T]) JsonUnmarshalUnchecked() *T {
	if err := r.AsError(); err != nil {
		return nil
	}
	var t T
	if r.Len() > 0 {
		_ = sonic.Unmarshal(r.AsBytes(), &t)
	}
	return &t
}

// Unmarshal unmarshal to object
// NOTE: maybe reference Rust memory buffer
//
//go:inline
func (r RustFfiResult[T]) Unmarshal(unmarshal func([]byte, any) error) (*T, error) {
	if err := r.AsError(); err != nil {
		return nil, err
	}
	var t T
	if r.Len() > 0 {
		err := unmarshal(r.AsBytes(), &t)
		if err != nil {
			return nil, err
		}
	}
	return &t, nil
}

// UnmarshalUnchecked unmarshal to object
// NOTE: maybe reference Rust memory buffer
//
//go:inline
func (r RustFfiResult[T]) UnmarshalUnchecked(unmarshal func([]byte, any) error) *T {
	if err := r.AsError(); err != nil {
		return nil
	}
	var t T
	if r.Len() > 0 {
		_ = unmarshal(r.AsBytes(), &t)
	}
	return &t
}

"###
        .to_string()
    }

    // main.go
    fn codegen_goffi_iface_method(&self, _def_id: DefId, method: &Arc<Method>) -> Option<String> {
        let mod_name = self.config.gomod_name.clone();
        let iface_method_name = self.iface_method_name(method);
        let args_sign = method
            .args
            .iter()
            .map(|arg| {
                if arg.ty.is_scalar() {
                    format!("{} {}", self.arg_name(arg), self.arg_type(arg, true))
                } else {
                    format!(
                        "{} {mod_name}.TBytes[{}]",
                        self.arg_name(arg),
                        self.arg_type(arg, true)
                    )
                }
            })
            .collect::<Vec<String>>()
            .join(",");
        let ret_type = self.ret_type(method, true);
        let is_empty_ret = self.context.is_empty_ty(&method.ret.kind);
        Some(if is_empty_ret {
            format!("{iface_method_name}({args_sign}) ResultMsg")
        } else {
            format!("{iface_method_name}({args_sign}) gust.EnumResult[{mod_name}.TBytes[*{ret_type}], ResultMsg]")
        })
    }

    // main.go
    fn codegen_goffi_service_impl(&self, service_def_id: DefId, s: &Service) -> String {
        let mod_name = self.config.gomod_name.clone();
        let mut ffi_functions = String::new();

        for method in &s.methods {
            let is_empty_ret = self.context.is_empty_ty(&method.ret.kind);
            let iface_method_name = self.iface_method_name(method);
            let ffi_func_name = self.ffi_func_name(service_def_id, method);
            let ffi_args_assign = method
                .args
                .iter()
                .map(|arg| {
                    if arg.ty.is_scalar() {
                        let name = self.arg_name(arg);
                        if let TyKind::Bool = arg.ty.kind {
                            format!("bool({name})")
                        } else {
                            name
                        }
                    } else {
                        format!(
                            "asBytes[{}]({})",
                            self.arg_type(arg, true),
                            self.arg_name(arg)
                        )
                    }
                })
                .collect::<Vec<String>>()
                .join(",");
            let ffi_args_sign = method
                .args
                .iter()
                .map(|arg| {
                    if arg.ty.is_scalar() {
                        format!("{} {}", self.arg_name(arg), self.arg_type(arg, true))
                    } else {
                        format!("{} C.struct_Buffer", self.arg_name(arg))
                    }
                })
                .collect::<Vec<String>>()
                .join(",");

            if is_empty_ret {
                ffi_functions.push_str(&format!(r###"
                //go:inline
                //export {ffi_func_name}
                func {ffi_func_name}({ffi_args_sign}) C.struct_GoFfiResult {{
                    if _{iface_method_name}_Ret_Msg := GlobalGoFfi.{iface_method_name}({ffi_args_assign}); _{iface_method_name}_Ret_Msg.Code == {mod_name}.RcNoError {{
                        return C.struct_GoFfiResult{{}}
                    }} else {{
                        return C.struct_GoFfiResult{{
                            code:     C.int8_t(_{iface_method_name}_Ret_Msg.Code),
                            data_ptr: C.leak_buffer(asBuffer({mod_name}.TBytesFromString[string](_{iface_method_name}_Ret_Msg.Msg))),
                        }}
                    }}
                }}
                "###));
            } else {
                ffi_functions.push_str(&format!(r###"
                //go:inline
                //export {ffi_func_name}
                func {ffi_func_name}({ffi_args_sign}) C.struct_GoFfiResult {{
                    if _{iface_method_name}_Ret := GlobalGoFfi.{iface_method_name}({ffi_args_assign}); _{iface_method_name}_Ret.IsOk() {{
                        return C.{ffi_func_name}_set_result(asBuffer(_{iface_method_name}_Ret.Unwrap()))
                    }} else {{
                        _{iface_method_name}_Ret_Msg := _{iface_method_name}_Ret.UnwrapErr()
                        if _{iface_method_name}_Ret_Msg.Code == {mod_name}.RcNoError {{
                            _{iface_method_name}_Ret_Msg.Code = {mod_name}.RcUnknown
                        }}
                        return C.struct_GoFfiResult{{
                            code:     C.int8_t(_{iface_method_name}_Ret_Msg.Code),
                            data_ptr: C.leak_buffer(asBuffer({mod_name}.TBytesFromString[string](_{iface_method_name}_Ret_Msg.Msg))),
                        }}
                    }}
                }}
                "###));
            }
        }

        format!(
            r###"
        type ResultMsg struct {{
            Code {mod_name}.ResultCode
            Msg  string
        }}

        //go:inline
        func asBuffer[T any](b {mod_name}.TBytes[T]) C.struct_Buffer {{
            p, size := b.ForCBuffer()
            if size == 0 {{
                return C.struct_Buffer{{}}
            }}
            return C.struct_Buffer{{
                ptr: (*C.uint8_t)(p),
                len: C.uintptr_t(size),
                cap: C.uintptr_t(size),
            }}
        }}

        //go:inline
        func asBytes[T any](buf C.struct_Buffer) {mod_name}.TBytes[T] {{
            if buf.len == 0 {{
                return {mod_name}.TBytes[T]{{}}
            }}
            return {mod_name}.TBytesFromBytes[T](*(*[]byte)(unsafe.Pointer(&reflect.SliceHeader{{
                Data: uintptr(unsafe.Pointer(buf.ptr)),
                Len:  int(buf.len),
                Cap:  int(buf.cap),
            }})))
        }}

        {ffi_functions}
        "###
        )
    }
}
