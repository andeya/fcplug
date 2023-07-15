package ctypes

import (
	"runtime"
	"sync"
	"unsafe"
)

// $ CGO_ENABLED=1 go build -buildmode=c-archive -o callee.a
var escapeGC = sync.Map{}

type C_DynArray[T any] struct {
	Ptr uintptr
	Len int
	Cap int
}

func Null[T any]() C_DynArray[T] {
	return C_DynArray[T]{}
}

//go:inline
//go:nosplit
func FromVec[T any](a []T) C_DynArray[T] {
	if len(a) == 0 {
		return C_DynArray[T]{}
	}
	vec := *(*C_DynArray[T])(unsafe.Pointer(&a))
	escapeGC.Store(vec.Ptr, a)
	runtime.KeepAlive(a)
	return vec
}

//go:inline
//go:nosplit
func (c *C_DynArray[T]) IsEmpty() bool {
	return c == nil || c.Ptr == 0 || c.Len == 0 || c.Cap == 0
}

//go:inline
//go:nosplit
func (c *C_DynArray[T]) FreeGC() {
	if c != nil && c.Ptr > 0 {
		escapeGC.Delete(c.Ptr)
		c.Ptr = 0
		c.Len = 0
		c.Cap = 0
	}
}

//go:inline
//go:nosplit
func (c *C_DynArray[T]) IntoVec() []T {
	if c != nil {
		a := *(*[]T)(unsafe.Pointer(c))
		c.FreeGC()
		return a
	}
	return nil
}

//go:inline
//go:nosplit
func FromString[STRING ~string](s STRING) C_DynArray[byte] {
	if len(s) == 0 {
		return C_DynArray[byte]{}
	}
	vec := *(*C_DynArray[byte])(unsafe.Pointer(&struct {
		string
		Cap int
	}{*(*string)(unsafe.Pointer(&s)), len(s)}))
	escapeGC.Store(vec.Ptr, s)
	runtime.KeepAlive(s)
	return vec
}

//go:inline
//go:nosplit
func IntoString[STRING ~string](c *C_DynArray[byte]) STRING {
	a := c.IntoVec()
	if len(a) == 0 {
		return ""
	}
	return *(*STRING)(unsafe.Pointer(&a))
}

type MapEntry[K comparable, V any] struct {
	Key   K
	Value V
}

type Map[K comparable, V any] []MapEntry[K, V]
type C_Map[K comparable, V any] C_DynArray[MapEntry[K, V]]

//go:inline
//go:nosplit
func FromMap[K comparable, V any](m map[K]V) C_DynArray[MapEntry[K, V]] {
	if len(m) == 0 {
		return C_DynArray[MapEntry[K, V]]{}
	}
	a := make([]MapEntry[K, V], 0, len(m))
	for k, v := range m {
		a = append(a, MapEntry[K, V]{Key: k, Value: v})
	}
	return FromVec(a)
}

//go:inline
//go:nosplit
func FromMap2[K comparable, V any](m map[K]V) C_Map[K, V] {
	return C_Map[K, V](FromMap(m))
}

//go:inline
//go:nosplit
func IntoMap[K comparable, V any](c *C_DynArray[MapEntry[K, V]]) map[K]V {
	a := c.IntoVec()
	if len(a) == 0 {
		return nil
	}
	m := make(map[K]V, len(a))
	for _, b := range a {
		m[b.Key] = b.Value
	}
	return m
}

//go:inline
//go:nosplit
func IntoMap2[K comparable, V any](c *C_Map[K, V]) map[K]V {
	return IntoMap((*C_DynArray[MapEntry[K, V]])(c))
}

type ConvReprGo[ReprC any, ReprGo any] interface {
	FromReprGo(cObject ReprGo)
	IntoReprGo() ReprGo
}

var _ ConvReprGo[C_Map[string, string], map[string]string] = &C_Map[string, string]{}

func (c *C_Map[K, V]) FromReprGo(cObject map[K]V) {
	if c == nil || cObject == nil {
		return
	}
	// TODO implement me
	panic("implement me")
}

func (c C_Map[K, V]) IntoReprGo() map[K]V {
	// TODO implement me
	panic("implement me")
}

func IntoReprC[ReprGo any, ReprC any](goObject ReprGo) ReprC {
	panic(nil)
}

func FromReprC[ReprGo any, ReprC any](cObject ReprC) ReprGo {
	panic(nil)
}
