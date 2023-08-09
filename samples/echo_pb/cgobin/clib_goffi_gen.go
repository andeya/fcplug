// Code generated by fcplug. DO NOT EDIT.

package main

/*
   #cgo CFLAGS: -I/Users/henrylee2cn/rust/fcplug/target/debug
   #cgo LDFLAGS: -L/Users/henrylee2cn/rust/fcplug/target/debug -lecho_pb

   #include "echo_pb.h"
*/
import "C"
import (
	"reflect"
	"unsafe"

	"github.com/andeya/fcplug/samples/echo_pb"
	"github.com/andeya/gust"
)

// main function is never called by C to.
func main() {}

var (
	_ reflect.SliceHeader
	_ unsafe.Pointer
	_ gust.EnumResult[any, any]
	_ echo_pb.ResultCode
)

var GlobalGoFfi GoFfi = _UnimplementedGoFfi{}

type ResultMsg struct {
	Code echo_pb.ResultCode
	Msg  string
}

//go:inline
func asBuffer[T any](b echo_pb.TBytes[T]) C.struct_Buffer {
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
func asBytes[T any](buf C.struct_Buffer) echo_pb.TBytes[T] {
	if buf.len == 0 {
		return echo_pb.TBytes[T]{}
	}
	return echo_pb.TBytesFromBytes[T](*(*[]byte)(unsafe.Pointer(&reflect.SliceHeader{
		Data: uintptr(unsafe.Pointer(buf.ptr)),
		Len:  int(buf.len),
		Cap:  int(buf.cap),
	})))
}

type GoFfi interface {
	EchoGo(req echo_pb.TBytes[echo_pb.Ping]) gust.EnumResult[echo_pb.TBytes[*echo_pb.Pong], ResultMsg]
}
type _UnimplementedGoFfi struct{}

func (_UnimplementedGoFfi) EchoGo(req echo_pb.TBytes[echo_pb.Ping]) gust.EnumResult[echo_pb.TBytes[*echo_pb.Pong], ResultMsg] {
	panic("unimplemented")
}

//go:inline
//export goffi_echo_go
func goffi_echo_go(req C.struct_Buffer) C.struct_GoFfiResult {
	if _EchoGo_Ret := GlobalGoFfi.EchoGo(asBytes[echo_pb.Ping](req)); _EchoGo_Ret.IsOk() {
		return C.goffi_echo_go_set_result(asBuffer(_EchoGo_Ret.Unwrap()))
	} else {
		_EchoGo_Ret_Msg := _EchoGo_Ret.UnwrapErr()
		if _EchoGo_Ret_Msg.Code == echo_pb.RcNoError {
			_EchoGo_Ret_Msg.Code = echo_pb.RcUnknown
		}
		return C.struct_GoFfiResult{
			code:     C.int8_t(_EchoGo_Ret_Msg.Code),
			data_ptr: C.leak_buffer(asBuffer(echo_pb.TBytesFromString[string](_EchoGo_Ret_Msg.Msg))),
		}
	}
}