package main

import (
	"github.com/andeya/fcplug/samples/echo"
	"github.com/andeya/gust"
)

func init() {
	// TODO: Replace with your own implementation, then re-execute `cargo build`
	GlobalGoFfi = GoFfiImpl{}
}

type GoFfiImpl struct{}

func (g GoFfiImpl) EchoGo(req echo.TBytes[echo.Ping]) gust.EnumResult[echo.TBytes[*echo.Pong], ResultMsg] {
	_ = req.PbUnmarshalUnchecked()
	// fmt.Printf("go receive req: %v\n", req.PbUnmarshalUnchecked())
	return gust.EnumOk[echo.TBytes[*echo.Pong], ResultMsg](echo.TBytesFromPbUnchecked(&echo.Pong{
		Msg: "this is pong from go",
	}))
}
