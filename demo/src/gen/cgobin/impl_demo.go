package main

import (
	"github.com/andeya/fcplug/demo/src/gen"
	"github.com/andeya/gust"
)

func init() {
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

func (t Test) SearchClient(g gen.TBytes[*gen.SearchRequest]) gust.EnumResult[gen.TBytes[*gen.Client], ResultMsg] {
	return gust.EnumOk[gen.TBytes[*gen.Client], ResultMsg](gen.TBytesFromPbUnchecked(&gen.Client{
		Ip:   "127.0.0.1",
		City: "shenzhen",
	}))
}
