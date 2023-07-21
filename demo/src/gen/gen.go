package gen

/*
   #cgo CFLAGS: -I/Users/henrylee2cn/rust/fcplug/target/debug
   #cgo LDFLAGS: -L/Users/henrylee2cn/rust/fcplug/target/debug -ldemo

   #include "demo.h"
*/
import "C"

import (
	"errors"
	"reflect"
	"unsafe"

	"github.com/andeya/gust/valconv"
	"github.com/bytedance/sonic"
	"github.com/golang/protobuf/proto"
)

var (
	_ = errors.New
	_ reflect.SliceHeader
	_ unsafe.Pointer
	_ valconv.ReadonlyBytes
	_ = sonic.Marshal
	_ = proto.Marshal
)

var GlobalRustFfi RustFfi = RustFfiImpl{}

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
//
//go:inline
func TBytesFromBytes[T any](bytes []byte) TBytes[T] {
	return TBytes[T]{bytes: bytes}
}

// TBytesFromString new TBytes from string
//
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
	return valconv.BytesToString[string](b.AsBytes())
}

//go:inline
func (b CBuffer) String() string {
	return b.AsString()
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

type RustFfi interface {
	SearchWebSite(req TBytes[*SearchRequest]) RustFfiResult[WebSite]
}
type RustFfiImpl struct{}

//go:inline
func (RustFfiImpl) SearchWebSite(req TBytes[*SearchRequest]) RustFfiResult[WebSite] {
	return newRustFfiResult[WebSite](C.rustffi_search_web_site(req.asBuffer()))
}