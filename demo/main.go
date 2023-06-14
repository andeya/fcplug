package main

import (
	"github.com/andeya/fcplug/demo/go_gen"
	"github.com/andeya/fcplug/go/gocall"
	"github.com/davecgh/go-spew/spew"
	flatbuffers "github.com/google/flatbuffers/go"
)

func main() {
	res, free := go_gen.C_ffi_raw_echo([]byte("hello raw"))
	_, _ = spew.Printf("C_ffi_raw_echo: code=%d, data=%q\n", res.Code, res.Data)
	free()

	pbRes := go_gen.C_ffi_pb_echo[*go_gen.Echo](&go_gen.Echo{
		Msg: "hello protobuf",
	})
	_, _ = spew.Printf("C_ffi_pb_echo: %#v\n", pbRes)
	fbRes := C_ffi_fb_echo("hello flatbuf")
	_, _ = spew.Printf("C_ffi_fb_echo: %#v\n", fbRes)
}

func C_ffi_fb_echo(req string) *gocall.ABIResult[string] {
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
	return &x
}
