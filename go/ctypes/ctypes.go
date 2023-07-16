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
func (c *C_DynArray[T]) IntoSlice() []T {
	if c != nil {
		a := *(*[]T)(unsafe.Pointer(c))
		c.FreeGC()
		return a
	}
	return nil
}

//go:inline
//go:nosplit
func Null[T any]() C_DynArray[T] {
	return C_DynArray[T]{}
}

//go:inline
//go:nosplit
func SliceReprGoToC[G any, C any](gslice []G, convElemFunc func(G) C) C_DynArray[C] {
	if len(gslice) == 0 {
		return C_DynArray[C]{}
	}
	if convElemFunc == nil {
		cslice := *(*C_DynArray[C])(unsafe.Pointer(&gslice))
		escapeGC.Store(cslice.Ptr, gslice)
		runtime.KeepAlive(gslice)
		return cslice
	}
	cslice := make([]C, len(gslice))
	for i, t := range gslice {
		cslice[i] = convElemFunc(t)
	}
	vec := *(*C_DynArray[C])(unsafe.Pointer(&cslice))
	escapeGC.Store(vec.Ptr, cslice)
	runtime.KeepAlive(cslice)
	return vec
}

//go:inline
//go:nosplit
func SliceReprCToGo[C any, G any](cslice *C_DynArray[C], convElemFunc func(*C) G) []G {
	if cslice.IsEmpty() {
		return nil
	}
	if convElemFunc == nil {
		gslice := *(*[]G)(unsafe.Pointer(cslice))
		cslice.FreeGC()
		return gslice
	}
	a := *(*[]C)(unsafe.Pointer(cslice))
	cslice.FreeGC()
	gslice := make([]G, len(a))
	for i, t := range a {
		gslice[i] = convElemFunc(&t)
	}
	return gslice
}

type C_String C_DynArray[byte]

func (c *C_String) Downcast() *C_DynArray[byte] {
	return (*C_DynArray[byte])(c)
}

//go:inline
//go:nosplit
func StringReprCToGo[STRING ~string](cstr *C_String) STRING {
	gslice := SliceReprCToGo[byte, byte](cstr.Downcast(), nil)
	if len(gslice) == 0 {
		return ""
	}
	return *(*STRING)(unsafe.Pointer(&gslice))
}

//go:inline
//go:nosplit
func StringReprGoToC[STRING ~string](gstr STRING) C_String {
	if len(gstr) == 0 {
		return C_String{}
	}
	cslice := *(*C_DynArray[byte])(unsafe.Pointer(&struct {
		string
		Cap int
	}{*(*string)(unsafe.Pointer(&gstr)), len(gstr)}))
	escapeGC.Store(cslice.Ptr, gstr)
	runtime.KeepAlive(gstr)
	return C_String(cslice)
}

type MapEntry[K comparable, V any] struct {
	Key   K
	Value V
}

type Map[K comparable, V any] []MapEntry[K, V]
type C_Map[K comparable, V any] C_DynArray[MapEntry[K, V]]

func (c *C_Map[K, V]) Downcast() *C_DynArray[MapEntry[K, V]] {
	return (*C_DynArray[MapEntry[K, V]])(c)
}

//go:inline
//go:nosplit
func MapReprCToGo[CK comparable, CV any, GK comparable, GV any](cmap *C_Map[CK, CV], convK func(*CK) GK, convV func(*CV) GV) map[GK]GV {
	a := SliceReprCToGo[MapEntry[CK, CV], MapEntry[CK, CV]](cmap.Downcast(), nil)
	if len(a) == 0 {
		return nil
	}
	if convK == nil {
		convK = func(ck *CK) GK {
			return *(*GK)(unsafe.Pointer(ck))
		}
	}
	if convV == nil {
		convV = func(cv *CV) GV {
			return *(*GV)(unsafe.Pointer(cv))
		}
	}
	gmap := make(map[GK]GV, len(a))
	for _, entry := range a {
		gmap[convK(&entry.Key)] = convV(&entry.Value)
	}
	return gmap
}

//go:inline
//go:nosplit
func MapReprGoToC[GK comparable, GV any, CK comparable, CV any](gmap map[GK]GV, convK func(GK) CK, convV func(GV) CV) C_Map[CK, CV] {
	if len(gmap) == 0 {
		return C_Map[CK, CV]{}
	}
	if convK == nil {
		convK = func(gk GK) CK {
			return *(*CK)(unsafe.Pointer(&gk))
		}
	}
	if convV == nil {
		convV = func(gv GV) CV {
			return *(*CV)(unsafe.Pointer(&gv))
		}
	}
	cmap := make([]MapEntry[CK, CV], 0, len(gmap))
	for k, v := range gmap {
		cmap = append(cmap, MapEntry[CK, CV]{
			Key:   convK(k),
			Value: convV(v),
		})
	}
	return C_Map[CK, CV](SliceReprGoToC[MapEntry[CK, CV], MapEntry[CK, CV]](cmap, nil))
}
