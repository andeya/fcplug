package main

import (
	"github.com/andeya/fcplug/demo/src/gen"
	"github.com/andeya/gust"
)

func init() {
	GlobalGoFfi = Test{}
}

type Test struct{}

func (t Test) TestEmpty(req gen.TBytes[*gen.Empty]) gust.EnumResult[gen.TBytes[*gen.Empty], ResultMsg] {
	// return gust.EnumOk[gen.TBytes[*gen.Empty], ResultMsg](gen.TBytes[*gen.Empty]{})
	return gust.EnumErr[gen.TBytes[*gen.Empty], ResultMsg](ResultMsg{
		Code: 1,
		Msg:  "empty test error",
	})
}

func (t Test) SearchClient(g gen.TBytes[*gen.SearchRequest]) gust.EnumResult[gen.TBytes[*gen.Client], ResultMsg] {
	return gust.EnumOk[gen.TBytes[*gen.Client], ResultMsg](gen.TBytesFromPbUnchecked(&gen.Client{
		Ip:   "127.0.0.1",
		City: "shenzhen",
	}))
}
