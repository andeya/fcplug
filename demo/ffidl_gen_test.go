package demo

import (
	"testing"

	"github.com/davecgh/go-spew/spew"
)

func TestRustFfiSearchWebSite(t *testing.T) {
	ret := GlobalRustFfi.SearchWebSite(TBytesFromPbUnchecked(&SearchRequest{
		Query:         "abc",
		PageNumber:    70,
		ResultPerPage: 14,
	}))
	defer ret.Free()
	t.Log(ret)
	obj, err := ret.PbUnmarshal()
	if err != nil {
		t.Fatal(err)
	} else {
		spew.Dump(obj)
	}
}

func TestEmpty(t *testing.T) {
	ret := GlobalRustFfi.RsTestEmpty()
	defer ret.Free()
	t.Log(ret)
	obj, err := ret.PbUnmarshal()
	if err != nil {
		t.Log(err)
	} else {
		spew.Dump(obj)
	}
}
