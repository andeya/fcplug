package echo_test

import (
	"testing"

	"github.com/andeya/fcplug/samples/echo"
)

func TestEcho(t *testing.T) {
	ret := echo.GlobalRustFfi.EchoRs(echo.TBytesFromPbUnchecked[*echo.Ping](&echo.Ping{
		Msg: "this is ping from go",
	}))
	if ret.IsOk() {
		t.Logf("%#v", ret.PbUnmarshalUnchecked())
	} else {
		t.Logf("fail: err=%v", ret.AsError())
		return
	}
}

// func BenchmarkEcho(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		ret := echo.GlobalRustFfi.EchoRs(echo.TBytesFromPbUnchecked[*echo.Ping](&echo.Ping{
// 			Msg: "this is ping from go",
// 		}))
// 		if ret.IsOk() {
// 			_ = ret.PbUnmarshalUnchecked()
// 		} else {
// 			b.Logf("fail: err=%v", ret.AsError())
// 			return
// 		}
// 	}
// }
