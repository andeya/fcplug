package caller

import (
	"errors"
	"fmt"
	"reflect"

	"github.com/golang/protobuf/proto"
)

type PbMessage = proto.Message

type ABIResult[T any] struct {
	Code ResultCode `json:"code,omitempty"`
	Data T          `json:"data,omitempty"`
}

func (a *ABIResult[T]) IsErr() bool {
	return a != nil && a.Code.IsErr()
}

func (a *ABIResult[T]) ToErr() error {
	if a == nil {
		return nil
	}
	return a.Code.Error()
}

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

func PbMarshal(m proto.Message) ([]byte, ResultCode) {
	b, err := proto.Marshal(m)
	if err != nil {
		return nil, CodeMarshal
	}
	return b, CodeNoError
}

func PbUnmarshal[T proto.Message](b []byte) ABIResult[T] {
	var m T
	if t := reflect.TypeOf(m); t.Kind() == reflect.Ptr {
		m = reflect.New(t.Elem()).Interface().(T)
	}
	err := proto.Unmarshal(b, m)
	if err != nil {
		return ABIResult[T]{
			Code: CodeUnmarshal,
		}
	}
	return ABIResult[T]{
		Data: m,
	}
}
