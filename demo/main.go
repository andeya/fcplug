package main

/*
#cgo CFLAGS: -I/
#cgo LDFLAGS: -L. -ldemo

#include "demo.h"
*/
import "C"
import (
	"fmt"
	_ "unsafe"

	"github.com/andeya/fcplug/demo/go_gen"
	"github.com/davecgh/go-spew/spew"
)

func main() {
	fmt.Printf("%T\n", C.ffi_echo)
	var r = InvokeFFI[*go_gen.Echo](func(args Buffer) Buffer { return C.ffi_echo(args) }, &go_gen.Echo{
		Msg: "hello andeya",
	})
	_, _ = spew.Printf("%#v\n", r)
}
