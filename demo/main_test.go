package main_test

import (
	"testing"

	"github.com/andeya/fcplug/demo/go_gen"
	"github.com/andeya/fcplug/go/gocall"
	"github.com/golang/protobuf/proto"
	flatbuffers "github.com/google/flatbuffers/go"
)

func BenchmarkEcho_Rust_fb(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = C_ffi_fb_echo("hello fcplug")
	}
}

func BenchmarkEcho_Rust_pb(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = go_gen.C_ffi_pb_echo[*go_gen.Echo](&go_gen.Echo{
			Msg: "hello fcplug",
		})
	}
}

func BenchmarkEcho_Go_pb(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = echo_go_pb(&go_gen.Echo{
			Msg: "hello fcplug",
		})
	}
}

func BenchmarkEcho_Go_raw(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = echo_go(&go_gen.Echo{
			Msg: "hello fcplug",
		})
	}
}

func TestEcho_rust_fb(t *testing.T) {
	t.Logf("%v", C_ffi_fb_echo("hello fcplug"))
}

func C_ffi_fb_echo(req string) gocall.ABIResult[string] {
	fbb := flatbuffers.NewBuilder(128)
	data := fbb.CreateString(req)
	go_gen.EchoRequestStart(fbb)
	go_gen.EchoRequestAddData(fbb, data)
	fbb.Finish(go_gen.EchoRequestEnd(fbb))

	res, free := go_gen.C_ffi_fb_echo_bytes(fbb.FinishedBytes())
	defer free()
	var x gocall.ABIResult[string]
	if res.IsErr() {
		x.Code = res.Code
	} else {
		var r go_gen.EchoResponse
		flatbuffers.GetRootAs(res.Data, 0, &r)
		x.Data = string(r.Data())
	}
	return x
}

func echo_go(args *go_gen.Echo) *gocall.ABIResult[*go_gen.Echo] {
	var a go_gen.Echo
	var b go_gen.Echo
	b.Msg = "input is: " + a.GetMsg()
	var c go_gen.Echo
	c.Msg = b.Msg
	return &gocall.ABIResult[*go_gen.Echo]{
		Code: 0,
		Data: &c,
	}
}

func echo_go_pb(args *go_gen.Echo) *gocall.ABIResult[*go_gen.Echo] {
	var a go_gen.Echo
	proto.Unmarshal([]byte(args.String()), &a)
	var b go_gen.Echo
	b.Msg = "input is: " + a.GetMsg()
	var c go_gen.Echo

	proto.Unmarshal([]byte(b.String()), &c)
	return &gocall.ABIResult[*go_gen.Echo]{
		Code: 0,
		Data: &c,
	}
}
