package main_test

import (
	"flag"
	"fmt"
	"strings"
	"testing"

	"github.com/andeya/gust"
	"github.com/andeya/gust/valconv"
	"github.com/golang/protobuf/proto"
	flatbuffers "github.com/google/flatbuffers/go"

	"github.com/andeya/fcplug/demo/internal/rsffi_gen"
	"github.com/andeya/fcplug/go/caller"
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
		res := rsffi_gen.C_ffi_raw_echo(valconv.StringToReadonlyBytes(content)).Unwrap()
		r := res.AsString()
		_ = r
		res.Free()
	}
}

func BenchmarkEcho_Go_pb(b *testing.B) {
	content := lazyContent.TryGetValue().Unwrap()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		res := echo_go_pb(&rsffi_gen.Echo{
			Msg: content,
		}).Unwrap()
		r := res.GetMsg()
		_ = r
	}
}

func BenchmarkEcho_Fcplug_pb(b *testing.B) {
	content := lazyContent.TryGetValue().Unwrap()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		res := rsffi_gen.C_ffi_pb_echo[*rsffi_gen.Echo](&rsffi_gen.Echo{
			Msg: content,
		}).Unwrap()
		r := res.GetMsg()
		_ = r
	}
}

func BenchmarkEcho_Fcplug_fb(b *testing.B) {
	content := lazyContent.TryGetValue().Unwrap()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		res := C_ffi_fb_echo(content).Unwrap()
		r := valconv.BytesToString[string](res.AsData().Data())
		_ = r
		res.Free()
	}
}

func C_ffi_fb_echo(req string) gust.EnumResult[rsffi_gen.CFlatData[*rsffi_gen.EchoResponse], caller.ResultCode] {
	fbb := flatbuffers.NewBuilder(128)
	data := fbb.CreateString(req)
	rsffi_gen.EchoRequestStart(fbb)
	rsffi_gen.EchoRequestAddData(fbb, data)
	fbb.Finish(rsffi_gen.EchoRequestEnd(fbb))
	return rsffi_gen.C_ffi_fb_echo[*rsffi_gen.EchoResponse](fbb, rsffi_gen.GetRootAsEchoResponse)
}

func echo_go_raw(args string) string {
	return "input is: " + args
}

func echo_go_pb(args *rsffi_gen.Echo) gust.Result[*rsffi_gen.Echo] {
	var a rsffi_gen.Echo
	bytes, _ := proto.Marshal(args)
	err := proto.Unmarshal(bytes, &a)
	if err != nil {
		return gust.Err[*rsffi_gen.Echo](err)
	}
	var b rsffi_gen.Echo
	b.Msg = "input is: " + a.GetMsg()
	var c rsffi_gen.Echo
	bytes, _ = proto.Marshal(&b)
	err = proto.Unmarshal(bytes, &c)
	return gust.Ret[*rsffi_gen.Echo](&c, err)
}
