package main

import (
	"github.com/andeya/fcplug/samples/echo_pb"
	"github.com/andeya/gust"
)

func init() {
	// TODO: Replace with your own implementation, then re-execute `cargo build`
	GlobalGoFfi = GoFfiImpl{}
}

type GoFfiImpl struct{}

func (g GoFfiImpl) EchoGo(req echo_pb.TBytes[echo_pb.Ping]) gust.EnumResult[echo_pb.TBytes[*echo_pb.Pong], ResultMsg] {
	ping := req.PbUnmarshalUnchecked()
	if ping.Msg != "this is ping from rust" {
		panic("ping==============:" + ping.Msg)
	}
	// fmt.Printf("go receive req: %v\n", req.PbUnmarshalUnchecked())
	return gust.EnumOk[echo_pb.TBytes[*echo_pb.Pong], ResultMsg](echo_pb.TBytesFromPbUnchecked(&echo_pb.Pong{
		Msg: "this is pong from go",
	}))
}
