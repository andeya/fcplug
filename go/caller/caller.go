package caller

import (
	"errors"
	"fmt"
	"reflect"

	"github.com/andeya/gust"
	"github.com/golang/protobuf/proto"
	flatbuffers "github.com/google/flatbuffers/go"
)

type (
	FlatBuffer   = flatbuffers.FlatBuffer
	FlatBuilder  = flatbuffers.Builder
	FlatUOffsetT = flatbuffers.UOffsetT
)

type (
	PbMessage = proto.Message
)

type ResultCode int32

const (
	CodeNoError   ResultCode = 0
	CodeDecode    ResultCode = 1
	CodeEncode    ResultCode = 2
	CodeUnknown   ResultCode = -1
	CodeUnmarshal ResultCode = -2
	CodeMarshal   ResultCode = -3
)

func (r ResultCode) IsErr() bool {
	return r != CodeNoError
}

func (r ResultCode) Error() error {
	switch r {
	case CodeNoError:
		return nil
	case CodeDecode:
		return errors.New("decode error")
	case CodeEncode:
		return errors.New("encode error")
	case CodeUnmarshal:
		return errors.New("unmarshal error")
	case CodeMarshal:
		return errors.New("marshal error")
	default:
		return fmt.Errorf("unknown ResultCode=%d", r)
	}
}

func PbMarshal(m proto.Message) gust.EnumResult[[]byte, ResultCode] {
	b, err := proto.Marshal(m)
	if err != nil {
		return gust.EnumErr[[]byte, ResultCode](CodeMarshal)
	}
	return gust.EnumOk[[]byte, ResultCode](b)
}

func PbUnmarshal[T proto.Message](b []byte) gust.EnumResult[T, ResultCode] {
	var m T
	if t := reflect.TypeOf(m); t.Kind() == reflect.Ptr {
		m = reflect.New(t.Elem()).Interface().(T)
	}
	err := proto.Unmarshal(b, m)
	if err != nil {
		return gust.EnumErr[T, ResultCode](CodeUnmarshal)
	}
	return gust.EnumOk[T, ResultCode](m)
}
