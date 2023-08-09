package main

import (
	"github.com/andeya/fcplug/samples/echo_thrift"
	"github.com/andeya/gust"
)

func init() {
	// TODO: Replace with your own implementation, then re-execute `cargo build`
	GlobalGoFfi = GoFfiImpl{}
}

type GoFfiImpl struct{}

func (g GoFfiImpl) EchoGo(req echo_thrift.TBytes[echo_thrift.Ping]) gust.EnumResult[echo_thrift.TBytes[*echo_thrift.Pong], ResultMsg] {
	_ = req.PbUnmarshalUnchecked()
	// fmt.Printf("go receive req: %v\n", req.PbUnmarshalUnchecked())
	return gust.EnumOk[echo_thrift.TBytes[*echo_thrift.Pong], ResultMsg](echo_thrift.TBytesFromPbUnchecked(&echo_thrift.Pong{
		Msg: "this is pong from go",
	}))
}
