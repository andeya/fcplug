package echo_thrift_test

import (
	"testing"

	"github.com/andeya/fcplug/samples/echo_thrift"
)

func TestEcho(t *testing.T) {
	ret := echo_thrift.GlobalRustFfi.EchoRs(echo_thrift.TBytesFromPbUnchecked[*echo_thrift.Ping](&echo_thrift.Ping{
		Msg: "this is ping from go",
	}))
	if ret.IsOk() {
		t.Logf("%#v", ret.PbUnmarshalUnchecked())
	} else {
		t.Logf("fail: err=%v", ret.AsError())
	}
	ret.Free()
}

func BenchmarkEcho(b *testing.B) {
	args := echo_thrift.TBytesFromPbUnchecked[*echo_thrift.Ping](&echo_thrift.Ping{
		Msg: "this is ping from go",
	})
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		ret := echo_thrift.GlobalRustFfi.EchoRs(args)
		if ret.IsOk() {
			_ = ret.AsBytes()
		} else {
			b.Logf("fail: err=%v", ret.AsError())
			return
		}
		ret.Free()
	}
}
