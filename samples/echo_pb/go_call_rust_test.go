package echo_pb_test

import (
	"testing"

	"github.com/andeya/fcplug/samples/echo_pb"
)

func TestEcho(t *testing.T) {
	ret := echo_pb.GlobalRustFfi.EchoRs(echo_pb.TBytesFromPbUnchecked[*echo_pb.Ping](&echo_pb.Ping{
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
	args := echo_pb.TBytesFromPbUnchecked[*echo_pb.Ping](&echo_pb.Ping{
		Msg: "this is ping from go",
	})
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		ret := echo_pb.GlobalRustFfi.EchoRs(args)
		if ret.IsOk() {
			_ = ret.AsBytes()
		} else {
			b.Logf("fail: err=%v", ret.AsError())
			return
		}
		ret.Free()
	}
}
