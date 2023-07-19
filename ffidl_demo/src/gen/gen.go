package gen

/*
   #cgo CFLAGS: -I/Users/henrylee2cn/rust/fcplug/target/debug
   #cgo LDFLAGS: -L/Users/henrylee2cn/rust/fcplug/target/debug -lffidl_demo

   #include "ffidl_demo.h"
*/
import "C"

import (
	"unsafe"

	"github.com/andeya/fcplug/go/ctypes"
)

var (
	_ unsafe.Pointer
	_ ctypes.FfiArray[any]
)
