package main_test

import (
	"testing"

	"github.com/andeya/fcplug/demo/go_gen"
	"github.com/andeya/fcplug/go/gocall"
	"github.com/golang/protobuf/proto"
)

func BenchmarkEcho_Rust(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = echo_rust(&go_gen.Echo{
			Msg: "hello liyachuan",
		})
	}
}

func BenchmarkEcho_Go(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = echo_go(&go_gen.Echo{
			Msg: "hello liyachuan",
		})
	}
}

func echo_rust(args *go_gen.Echo) *gocall.ABIResult[*go_gen.Echo] {
	return go_gen.C_ffi_echo[*go_gen.Echo](args)
}

func echo_go(args *go_gen.Echo) *gocall.ABIResult[*go_gen.Echo] {
	var a go_gen.Echo
	proto.Unmarshal([]byte(args.String()), &a)
	var b go_gen.Echo
	b.Msg = "input is: " + a.GetMsg()
	var c go_gen.Echo

	proto.Unmarshal([]byte(b.String()), &c)
	return &gocall.ABIResult[*go_gen.Echo]{
		Code: 0,
		Msg:  "",
		Data: &c,
	}
}
