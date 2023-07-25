use std::cell::RefCell;
use std::sync::Arc;

use pilota_build::{DefId, IdentName, rir::Service};
use pilota_build::rir::{Arg, Method};
use pilota_build::ty::TyKind;

use crate::ffidl::{Config, Cx, ServiceType};

#[derive(Clone)]
pub(crate) struct GoCodegenBackend {
    pub(crate) config: Arc<Config>,
    pub(crate) context: Cx,
    pub(crate) go_pkg_code: Arc<RefCell<String>>,
    pub(crate) go_main_code: Arc<RefCell<String>>,
}

impl GoCodegenBackend {
    pub(crate) fn codegen(&self, service_def_id: DefId, s: &Service) {
        match self.context.service_type(service_def_id) {
            ServiceType::RustFfi => {
                self.go_pkg_code.borrow_mut().push_str(&self.codegen_rust_ffi(service_def_id, s))
            }
            ServiceType::GoFfi => {
                self.go_main_code.borrow_mut().push_str(&self.codegen_go_ffi(service_def_id, s))
            }
        }
    }

    // {pkg}.go
    pub(crate) fn codegen_rust_ffi(&self, service_def_id: DefId, s: &Service) -> String {
        const FIXED_CODE: &'static str = r###"
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

"###;
        let mut iface_methods = String::new();
        let mut impl_methods = String::new();
        for method in &s.methods {
            let iface_method_name = self.iface_method_name(method);
            let args_sign = method.args.iter().map(|arg| {
                if arg.ty.is_scalar() {
                    format!("{} {}", self.arg_name(arg), self.arg_type(arg, false))
                } else {
                    format!("{} TBytes[*{}]", self.arg_name(arg), self.arg_type(arg, false))
                }
            }).collect::<Vec<String>>().join(",");
            let ret_type = self.ret_type(method, false);
            iface_methods.push_str(&format!("{iface_method_name}({args_sign}) RustFfiResult[{ret_type}]\n"));

            let ffi_func_name = self.ffi_func_name(service_def_id, method, true);
            let args_assign = method.args.iter().map(|arg| {
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
            }).collect::<Vec<String>>().join(",");
            impl_methods.push_str(&format!(r###"
            //go:inline
            func (RustFfiImpl) {iface_method_name}({args_sign}) RustFfiResult[{ret_type}] {{
                return newRustFfiResult[{ret_type}]({ffi_func_name}({args_assign}))
            }}
            "###));
        }
        format!(r###"
        var GlobalRustFfi RustFfi = RustFfiImpl{{}}

        {FIXED_CODE}

        type RustFfi interface {{
	        {iface_methods}
        }}
        type RustFfiImpl struct{{}}
        {impl_methods}
        "###)
    }

    // main.go
    pub(crate) fn codegen_go_ffi(&self, service_def_id: DefId, s: &Service) -> String {
        let mod_name = self.config.go_mod_name();
        let code = format!(r###"
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

"###);

        let mod_name = self.config.go_mod_name();
        let mut iface_methods = String::new();
        let mut impl_methods = String::new();
        let mut ffi_functions = String::new();

        for method in &s.methods {
            let iface_method_name = self.iface_method_name(method);
            let args_sign = method.args.iter().map(|arg| {
                if arg.ty.is_scalar() {
                    format!("{} {}", self.arg_name(arg), self.arg_type(arg, true))
                } else {
                    format!("{} {mod_name}.TBytes[*{}]", self.arg_name(arg), self.arg_type(arg, true))
                }
            }).collect::<Vec<String>>().join(",");
            let ret_type = self.ret_type(method, true);
            let is_empty_ret = self.context.is_empty_ty(&method.ret.kind);
            if is_empty_ret {
                iface_methods.push_str(&format!(r###"
                {iface_method_name}({args_sign}) ResultMsg
            "###));
                impl_methods.push_str(&format!(r###"
                func (_UnimplementedGoFfi) {iface_method_name}({args_sign}) ResultMsg {{
                    panic("unimplemented")
                }}
            "###));
            } else {
                iface_methods.push_str(&format!(r###"
                {iface_method_name}({args_sign}) gust.EnumResult[{mod_name}.TBytes[*{ret_type}], ResultMsg]
            "###));
                impl_methods.push_str(&format!(r###"
                func (_UnimplementedGoFfi) {iface_method_name}({args_sign}) gust.EnumResult[{mod_name}.TBytes[*{ret_type}], ResultMsg] {{
                    panic("unimplemented")
                }}
            "###));
            }

            let ffi_func_name = self.ffi_func_name(service_def_id, method, false);
            let ffi_args_assign = method.args.iter().map(|arg| {
                if arg.ty.is_scalar() {
                    let name = self.arg_name(arg);
                    if let TyKind::Bool = arg.ty.kind {
                        format!("bool({name})")
                    } else {
                        name
                    }
                } else {
                    format!("asBytes[*{}]({})", self.arg_type(arg, true), self.arg_name(arg))
                }
            }).collect::<Vec<String>>().join(",");
            let ffi_args_sign = method.args.iter().map(|arg| {
                if arg.ty.is_scalar() {
                    format!("{} {}", self.arg_name(arg), self.arg_type(arg, true))
                } else {
                    format!("{} C.struct_Buffer", self.arg_name(arg))
                }
            }).collect::<Vec<String>>().join(",");

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

        format!(r###"

        var GlobalGoFfi GoFfi = _UnimplementedGoFfi{{}}

        {code}

        type GoFfi interface {{
	        {iface_methods}
        }}
        type _UnimplementedGoFfi struct{{}}
        {impl_methods}

        {ffi_functions}
        "###)
    }

    #[inline]
    fn codegen_item_ty(&self, ty: &TyKind, is_main: bool) -> String {
        match &ty {
            TyKind::String => "string".to_string(),
            TyKind::Void => "struct{}".to_string(),
            TyKind::U8 => "uint8".to_string(),
            TyKind::Bool => "bool".to_string(),
            TyKind::Bytes => "[]byte".to_string(),
            TyKind::I8 => "int8".to_string(),
            TyKind::I16 => "int16".to_string(),
            TyKind::I32 => "int32".to_string(),
            TyKind::I64 => "int64".to_string(),
            TyKind::F64 => "float64".to_string(),
            TyKind::Vec(ty) => format!("[]{}", self.codegen_item_ty(&ty.kind, is_main)),
            TyKind::Set(ty) => format!("[]{}", self.codegen_item_ty(&ty.kind, is_main)),
            TyKind::Map(key, value) => format!("map[{}]{}", self.codegen_item_ty(&key.kind, is_main), self.codegen_item_ty(&value.kind, is_main)),
            TyKind::Path(path) => {
                let mut pkg_pre = String::new();
                if is_main {
                    pkg_pre = self.config.go_mod_name() + ".";
                }
                format!("{pkg_pre}{}", self.context.rust_name(path.did).0.to_string())
            }
            TyKind::UInt32 => "uint32".to_string(),
            TyKind::UInt64 => "uint64".to_string(),
            TyKind::F32 => "float32".to_string(),
            TyKind::Arc(ty) => format!("*{}", self.codegen_item_ty(&ty.kind, is_main)),
        }
    }
    fn iface_method_name(&self, method: &Arc<Method>) -> String {
        method.name.0.upper_camel_ident().to_string()
    }
    fn ffi_func_name(&self, service_def_id: DefId, method: &Arc<Method>, with_prefix: bool) -> String {
        let service_name_lower = self.context.rust_name(service_def_id).to_lowercase();
        let method_name_lower = (&**method.name).fn_ident();
        if with_prefix {
            format!("C.{service_name_lower}_{method_name_lower}")
        } else {
            format!("{service_name_lower}_{method_name_lower}")
        }
    }
    fn arg_name(&self, arg: &Arc<Arg>) -> String {
        arg.name.0.to_lowercase()
    }
    fn arg_type(&self, arg: &Arc<Arg>, is_main: bool) -> String {
        self.codegen_item_ty(&arg.ty.kind, is_main)
    }
    fn ret_type(&self, method: &Arc<Method>, is_main: bool) -> String {
        self.codegen_item_ty(&method.ret.kind, is_main)
    }
    // fn field_name(&self, f: &Arc<Field>) -> String {
    //     self.context.rust_name(f.did).0.upper_camel_ident().into_string()
    // }
    // fn field_tag(&self, f: &Arc<Field>) -> String {
    //     format!(r###"`json:"{}"`"###, self.context.rust_name(f.did).0.snake_ident())
    // }
    // fn struct_go_name(&self, message_def_id: DefId) -> String {
    //     self.context.rust_name(message_def_id).to_string()
    // }
}
