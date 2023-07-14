package main

// $ CGO_ENABLED=1 go build -buildmode=c-archive -o demo.a

/*
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Buffer {
  uint8_t *ptr;
  uintptr_t len;
  uintptr_t cap;
} Buffer;
*/
import "C"
import (
	"fmt"
	"reflect"
	"unsafe"
)

func bufferToBytes(buf C.struct_Buffer) []byte {
	return *(*[]byte)(unsafe.Pointer(&reflect.SliceHeader{
		Data: uintptr(unsafe.Pointer(buf.ptr)),
		Len:  int(buf.len),
		Cap:  int(buf.cap),
	}))
}

//export helloString
func helloString(buf C.struct_Buffer) {
	fmt.Println("input:", string(bufferToBytes(buf)))
}

//export helloString
func helloString2(buf C.struct_Buffer) C.Slice {
	fmt.Println("input:", string(bufferToBytes(buf)))
}
