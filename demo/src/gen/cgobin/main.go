package main

/*
#cgo CFLAGS: -I/Users/henrylee2cn/rust/fcplug/target/debug
#cgo LDFLAGS: -L/Users/henrylee2cn/rust/fcplug/target/debug -ldemo

#include "demo.h"
*/
import "C"
import (
	"reflect"
	"unsafe"

	"github.com/andeya/fcplug/demo/src/gen"
	"github.com/andeya/gust"
)

// main function is never called by C to.
func main() {}

var (
	_ reflect.SliceHeader
	_ unsafe.Pointer
	_ gust.EnumResult[any, any]
	_ gen.ResultCode
)

var GlobalGoFfi GoFfi = _UnimplementedGoFfi{}

type ResultMsg struct {
	Code gen.ResultCode
	Msg  string
}

//go:inline
func asBuffer[T any](b gen.TBytes[T]) C.struct_Buffer {
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

//go:inline
func asBytes[T any](buf C.struct_Buffer) gen.TBytes[T] {
	if buf.len == 0 {
		return gen.TBytes[T]{}
	}
	return gen.TBytesFromBytes[T](*(*[]byte)(unsafe.Pointer(&reflect.SliceHeader{
		Data: uintptr(unsafe.Pointer(buf.ptr)),
		Len:  int(buf.len),
		Cap:  int(buf.cap),
	})))
}

type GoFfi interface {
	SearchClient(req gen.TBytes[*gen.SearchRequest]) gust.EnumResult[gen.TBytes[*gen.Client], ResultMsg]

	TestEmpty() ResultMsg
}
type _UnimplementedGoFfi struct{}

func (_UnimplementedGoFfi) SearchClient(req gen.TBytes[*gen.SearchRequest]) gust.EnumResult[gen.TBytes[*gen.Client], ResultMsg] {
	panic("unimplemented")
}

func (_UnimplementedGoFfi) TestEmpty() ResultMsg {
	panic("unimplemented")
}

//go:inline
//export goffi_search_client
func goffi_search_client(req C.struct_Buffer) C.struct_GoFfiResult {
	if _SearchClient_Ret := GlobalGoFfi.SearchClient(asBytes[*gen.SearchRequest](req)); _SearchClient_Ret.IsOk() {
		return C.goffi_search_client_set_result(asBuffer(_SearchClient_Ret.Unwrap()))
	} else {
		_SearchClient_Ret_Msg := _SearchClient_Ret.UnwrapErr()
		if _SearchClient_Ret_Msg.Code == gen.RcNoError {
			_SearchClient_Ret_Msg.Code = gen.RcUnknown
		}
		return C.struct_GoFfiResult{
			code:     C.int8_t(_SearchClient_Ret_Msg.Code),
			data_ptr: C.leak_buffer(asBuffer(gen.TBytesFromString[string](_SearchClient_Ret_Msg.Msg))),
		}
	}
}

//go:inline
//export goffi_test_empty
func goffi_test_empty() C.struct_GoFfiResult {
	if _TestEmpty_Ret_Msg := GlobalGoFfi.TestEmpty(); _TestEmpty_Ret_Msg.Code == gen.RcNoError {
		return C.struct_GoFfiResult{}
	} else {
		return C.struct_GoFfiResult{
			code:     C.int8_t(_TestEmpty_Ret_Msg.Code),
			data_ptr: C.leak_buffer(asBuffer(gen.TBytesFromString[string](_TestEmpty_Ret_Msg.Msg))),
		}
	}
}
