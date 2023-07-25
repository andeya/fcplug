package main

import (
	"github.com/andeya/fcplug/demo"
	"github.com/andeya/gust"
)

func init() {
	// TODO: Replace with your own implementation, then re-execute `cargo build`
	GlobalGoFfi = Test{}
}

type Test struct{}

func (t Test) TestEmpty() ResultMsg {
	return ResultMsg{
		Code: 0,
		Msg:  "",
	}
	// return ResultMsg{
	// 	Code: 1,
	// 	Msg:  "empty test error",
	// }
}

func (t Test) SearchClient(g demo.TBytes[*demo.SearchRequest]) gust.EnumResult[demo.TBytes[*demo.Client], ResultMsg] {
	return gust.EnumOk[demo.TBytes[*demo.Client], ResultMsg](demo.TBytesFromPbUnchecked(&demo.Client{
		Ip:   "127.0.0.1",
		City: "shenzhen",
	}))
}
