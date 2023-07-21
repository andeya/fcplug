package main

/*
   #cgo CFLAGS: -I/Users/henrylee2cn/rust/fcplug/target/debug
   #cgo LDFLAGS: -L/Users/henrylee2cn/rust/fcplug/target/debug -lffidl_demo

   #include "ffidl_demo.h"
*/
import "C"
import (
	"reflect"
	"unsafe"

	"github.com/andeya/fcplug/ffidl_demo/src/gen"
	"github.com/andeya/gust"
	"github.com/andeya/gust/valconv"
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

func stringToBuffer(b string) C.struct_Buffer {
	return bytesToBuffer(valconv.StringToReadonlyBytes[string](b))
}

func bufferToString(buf C.struct_Buffer) string {
	return valconv.BytesToString[string](bufferToBytes(buf))
}

type ResultMsg struct {
	Code gen.ResultCode
	Msg  string
}

func toRustFfiResult(ret gust.EnumResult[[]byte, ResultMsg]) C.struct_RustFfiResult {
	if ret.IsOk() {
		return C.struct_RustFfiResult{
			code: 0,
			data: bytesToBuffer(ret.Unwrap()),
		}
	}
	err := ret.UnwrapErr()
	return C.struct_RustFfiResult{
		code: C.int8_t(err.Code),
		data: stringToBuffer(err.Msg),
	}
}

// type ABIResult[T] = gust.EnumResult[T, ResultMsg]

type GoFfi interface {
	SearchWebSite()
}

var UnimplementedGoFfi GoFfi = _UnimplementedGoFfi{}

type _UnimplementedGoFfi struct{}

//go:inline
func (_UnimplementedGoFfi) SearchWebSite() {
	panic("unimplemented")
}
