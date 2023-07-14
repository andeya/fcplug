package main

import (
	"github.com/andeya/gust"
	"github.com/davecgh/go-spew/spew"
	flatbuffers "github.com/google/flatbuffers/go"

	"github.com/andeya/fcplug/demo/internal/rsffi_gen"
	"github.com/andeya/fcplug/go/caller"
)

func main() {
	res := rsffi_gen.C_ffi_raw_echo([]byte("hello raw"))
	_, _ = spew.Printf("C_ffi_raw_echo: %v\n", res)
	res.Inspect(func(bytes rsffi_gen.CBytes) {
		bytes.Free()
	})
	pbRes := rsffi_gen.C_ffi_pb_echo[*rsffi_gen.Echo](&rsffi_gen.Echo{
		Msg: "hello protobuf",
	})
	_, _ = spew.Printf("C_ffi_pb_echo: %v\n", pbRes)
	C_ffi_fb_echo("hello flatbuf").Inspect(func(g rsffi_gen.CFlatData[*rsffi_gen.EchoResponse]) {
		_, _ = spew.Printf("C_ffi_fb_echo: %s\n", g.AsData().Data())
		g.Free()
	})
}

func C_ffi_fb_echo(req string) gust.EnumResult[rsffi_gen.CFlatData[*rsffi_gen.EchoResponse], caller.ResultCode] {
	fbb := flatbuffers.NewBuilder(128)
	data := fbb.CreateString(req)
	rsffi_gen.EchoRequestStart(fbb)
	rsffi_gen.EchoRequestAddData(fbb, data)
	fbb.Finish(rsffi_gen.EchoRequestEnd(fbb))
	return rsffi_gen.C_ffi_fb_echo[*rsffi_gen.EchoResponse](fbb, rsffi_gen.GetRootAsEchoResponse)
}
