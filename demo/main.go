package main

import (
	"github.com/andeya/fcplug/demo/go_gen"
	"github.com/davecgh/go-spew/spew"
)

func main() {
	r := go_gen.C_ffi_echo[*go_gen.Echo](&go_gen.Echo{
		Msg: "hello andeya",
	})
	_, _ = spew.Printf("%#v\n", r)
}
