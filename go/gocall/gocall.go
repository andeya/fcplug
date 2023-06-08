package gocall

import (
	"errors"
	"fmt"
	"reflect"

	"github.com/golang/protobuf/proto"
)

type ABIResult[T proto.Message] struct {
	Code ABICode `json:"code,omitempty"`
	Msg  string  `json:"msg,omitempty"`
	Data T       `json:"data,omitempty"`
}

func (a *ABIResult[T]) IsErr() bool {
	return a != nil && a.Code != 0
}

func (a *ABIResult[T]) ToErr() error {
	if a == nil {
		return errors.New("<nil>")
	}
	if a.Code != 0 {
		return errors.New(a.Msg)
	}
	return nil
}

type ABICode int32

const (
	OkCode             ABICode = 0
	ErrorCodeMarshal   ABICode = -1
	ErrorCodeUnmarshal ABICode = -2
)

func Marshal[T proto.Message](m proto.Message) ([]byte, *ABIResult[T]) {
	b, err := proto.Marshal(m)
	if err != nil {
		return nil, &ABIResult[T]{
			Code: ErrorCodeMarshal,
			Msg:  err.Error(),
		}
	}
	return b, nil
}

func Unmarshal[T proto.Message](b []byte) *ABIResult[T] {
	var res FFIResult
	err := proto.Unmarshal(b, &res)
	if err != nil {
		return &ABIResult[T]{
			Code: ErrorCodeUnmarshal,
			Msg:  err.Error(),
		}
	}
	if res.Code != 0 {
		return &ABIResult[T]{
			Code: ABICode(res.Code),
			Msg:  err.Error(),
		}
	}
	var m T
	if t := reflect.TypeOf(m); t.Kind() == reflect.Ptr {
		m = reflect.New(t.Elem()).Interface().(T)
	}
	err = proto.Unmarshal(res.GetData().GetValue(), m)
	if err != nil {
		return &ABIResult[T]{
			Code: ErrorCodeUnmarshal,
			Msg:  fmt.Sprintf("unmarshal: data=%s, error=%s", res.GetData().String(), err.Error()),
		}
	}
	return &ABIResult[T]{
		Code: ABICode(res.Code),
		Msg:  res.Msg,
		Data: m,
	}
}
