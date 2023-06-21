package main

import (
	"github.com/andeya/fcplug/demo/go_gen"
	"github.com/andeya/fcplug/go/caller"
	"github.com/andeya/gust"
	"github.com/davecgh/go-spew/spew"
	flatbuffers "github.com/google/flatbuffers/go"
)

func main() {
	res := go_gen.C_ffi_raw_echo([]byte("hello raw"))
	_, _ = spew.Printf("C_ffi_raw_echo: %v\n", res)
	res.Inspect(func(bytes go_gen.CBytes) {
		bytes.Free()
	})
	pbRes := go_gen.C_ffi_pb_echo[*go_gen.Echo](&go_gen.Echo{
		Msg: "hello protobuf",
	})
	_, _ = spew.Printf("C_ffi_pb_echo: %v\n", pbRes)
	C_ffi_fb_echo("hello flatbuf").Inspect(func(g go_gen.CFlatData[*go_gen.EchoResponse]) {
		_, _ = spew.Printf("C_ffi_fb_echo: %s\n", g.AsData().Data())
		g.Free()
	})
}

func C_ffi_fb_echo(req string) gust.EnumResult[go_gen.CFlatData[*go_gen.EchoResponse], caller.ResultCode] {
	fbb := flatbuffers.NewBuilder(128)
	data := fbb.CreateString(req)
	go_gen.EchoRequestStart(fbb)
	go_gen.EchoRequestAddData(fbb, data)
	fbb.Finish(go_gen.EchoRequestEnd(fbb))
	return go_gen.C_ffi_fb_echo[*go_gen.EchoResponse](fbb, go_gen.GetRootAsEchoResponse)
}
