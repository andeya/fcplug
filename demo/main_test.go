package main_test

import (
	"flag"
	"fmt"
	"strings"
	"testing"

	"github.com/andeya/fcplug/demo/go_gen"
	"github.com/andeya/fcplug/go/caller"
	"github.com/andeya/gust"
	"github.com/andeya/gust/valconv"
	"github.com/golang/protobuf/proto"
	flatbuffers "github.com/google/flatbuffers/go"
)

var csize = flag.Uint("size", 10, "content size(B)")

var lazyContent = gust.NewLazyValue[string]().SetInitFunc(func() gust.Result[string] {
	flag.Parse()
	fmt.Printf("Content Size: %d B\n", *csize)
	return gust.Ok(strings.Repeat("?", int(*csize)))
})

func BenchmarkEcho_Go_raw(b *testing.B) {
	content := lazyContent.TryGetValue().Unwrap()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		r := echo_go_raw(content)
		_ = r
	}
}

func BenchmarkEcho_Fcplug_raw(b *testing.B) {
	content := lazyContent.TryGetValue().Unwrap()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		res, free := go_gen.C_ffi_raw_echo(valconv.StringToReadonlyBytes(content))
		if !res.IsErr() {
			r := valconv.BytesToString[string](res.Data)
			_ = r
		}
		free()
	}
}

func BenchmarkEcho_Go_pb(b *testing.B) {
	content := lazyContent.TryGetValue().Unwrap()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		res := echo_go_pb(&go_gen.Echo{
			Msg: content,
		})
		if !res.IsErr() {
			r := res.Data.GetMsg()
			_ = r
		}
	}
}

func BenchmarkEcho_Fcplug_pb(b *testing.B) {
	content := lazyContent.TryGetValue().Unwrap()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		res := go_gen.C_ffi_pb_echo[*go_gen.Echo](&go_gen.Echo{
			Msg: content,
		})
		if !res.IsErr() {
			r := res.Data.GetMsg()
			_ = r
		}
	}
}

func BenchmarkEcho_Fcplug_fb(b *testing.B) {
	content := lazyContent.TryGetValue().Unwrap()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		res, free := C_ffi_fb_echo(content)
		if !res.IsErr() {
			r := res.Data
			_ = r
		}
		free()
	}
}

func C_ffi_fb_echo(req string) (caller.ABIResult[string], func()) {
	fbb := flatbuffers.NewBuilder(128)
	data := fbb.CreateString(req)
	go_gen.EchoRequestStart(fbb)
	go_gen.EchoRequestAddData(fbb, data)
	fbb.Finish(go_gen.EchoRequestEnd(fbb))

	res, free := go_gen.C_ffi_fb_echo_bytes(fbb.FinishedBytes())
	var x caller.ABIResult[string]
	if res.IsErr() {
		x.Code = res.Code
	} else {
		var r go_gen.EchoResponse
		flatbuffers.GetRootAs(res.Data, 0, &r)
		x.Data = valconv.BytesToString[string](r.Data())
	}
	return x, free
}

func echo_go_raw(args string) string {
	return "input is: " + args
}

func echo_go_pb(args *go_gen.Echo) *caller.ABIResult[*go_gen.Echo] {
	var a go_gen.Echo
	proto.Unmarshal([]byte(args.String()), &a)
	var b go_gen.Echo
	b.Msg = "input is: " + a.GetMsg()
	var c go_gen.Echo

	proto.Unmarshal([]byte(b.String()), &c)
	return &caller.ABIResult[*go_gen.Echo]{
		Code: 0,
		Data: &c,
	}
}
