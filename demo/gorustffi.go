package main

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: -L. -ldemo

#include "demo.h"
*/
import "C"
import (
	"errors"
	"fmt"
	"reflect"
	"unsafe"

	"github.com/andeya/fcplug/go/gocall"
	"github.com/golang/protobuf/proto"
)

type Buffer = C.struct_Buffer
type Method = func(Buffer) Buffer
type ABICode int32
type ABIResult[T proto.Message] struct {
	Code ABICode `json:"code,omitempty"`
	Msg  string  `json:"msg,omitempty"`
	Data T       `json:"data,omitempty"`
}

func (a *ABIResult[T]) IsErr() bool {
	return a == nil || a.Code != 0
}

func (a *ABIResult[T]) ToErr() error {
	if a == nil {
		return errors.New("<nil>")
	}
	if a.Code != 0 {
		return errors.New(a.Msg)
	}
	return nil
}

const (
	OkCode             ABICode = 0
	ErrorCodeMarshal   ABICode = -1
	ErrorCodeUnmarshal ABICode = -2
)

func InvokeFFI[T proto.Message](method Method, args proto.Message) *ABIResult[T] {
	b, err := proto.Marshal(args)
	if err != nil {
		return &ABIResult[T]{
			Code: ErrorCodeMarshal,
			Msg:  err.Error(),
		}
	}
	argBuf := C.struct_Buffer{
		ptr: (*C.uint8_t)(unsafe.Pointer(&b[0])),
		len: C.uintptr_t(len(b)),
		cap: C.uintptr_t(cap(b)),
	}
	resBuf := method(argBuf)
	defer C.free_buffer(resBuf)
	return cBufferToResult[T](resBuf)
}

func cBufferToResult[T proto.Message](resBuf C.struct_Buffer) *ABIResult[T] {
	var res gocall.FFIResult
	err := proto.Unmarshal(cBufferToBytes(resBuf), &res)
	if err != nil {
		return &ABIResult[T]{
			Code: ErrorCodeUnmarshal,
			Msg:  err.Error(),
		}
	}
	if res.Code != 0 {
		return &ABIResult[T]{
			Code: ABICode(res.Code),
			Msg:  err.Error(),
		}
	}
	var m T
	if t := reflect.TypeOf(m); t.Kind() == reflect.Ptr {
		m = reflect.New(t.Elem()).Interface().(T)
	}
	err = proto.Unmarshal(res.GetData().GetValue(), m)
	if err != nil {
		return &ABIResult[T]{
			Code: ErrorCodeUnmarshal,
			Msg:  fmt.Sprintf("unmarshal: data=%s, error=%s", res.GetData().String(), err.Error()),
		}
	}
	return &ABIResult[T]{
		Code: ABICode(res.Code),
		Msg:  res.Msg,
		Data: m,
	}
}

func cBufferToBytes(buf C.struct_Buffer) []byte {
	return *(*[]byte)(unsafe.Pointer(&reflect.SliceHeader{
		Data: uintptr(unsafe.Pointer(buf.ptr)),
		Len:  int(buf.len),
		Cap:  int(buf.cap),
	}))
}
