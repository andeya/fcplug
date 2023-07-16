package ctypes

import (
	"runtime"
	"testing"

	"github.com/stretchr/testify/assert"
)

//go:noinline
func slice_gc() C_DynArray[string] {
	s := []string{"abc"}
	return SliceReprGoToC[string, string](s, nil)
}

//go:noinline
func TestSlice(t *testing.T) {
	s := []string{"abc"}
	cslice := slice_gc()
	runtime.GC()
	t.Log(s, cslice)
	runtime.GC()
	s2 := SliceReprCToGo[string, string](&cslice, nil)
	runtime.GC()
	t.Log(s, cslice, s2)
	s3 := SliceReprCToGo[string, string](&cslice, nil)
	runtime.GC()
	assert.Empty(t, s3)
}

//go:noinline
func string_gc() C_String {
	s := "abc"
	return StringReprGoToC[string](s)
}

//go:noinline
func TestString(t *testing.T) {
	s := "abc"
	cstring := string_gc()
	runtime.GC()
	t.Log(s, cstring)
	runtime.GC()
	s2 := StringReprCToGo[string](&cstring)
	runtime.GC()
	t.Log(s, cstring, s2)
	s3 := StringReprCToGo[string](&cstring)
	runtime.GC()
	assert.Empty(t, s3)
}

//go:noinline
func TestMap(t *testing.T) {
	s := map[string]int{"abc": 1}
	cmap := map_gc()
	runtime.GC()
	t.Log(s, cmap)
	runtime.GC()
	s2 := MapReprCToGo[C_String, int, string, int](&cmap, StringReprCToGo[string], nil)
	runtime.GC()
	t.Log(s, cmap, s2)
	s3 := MapReprCToGo[C_String, int, string, int](&cmap, StringReprCToGo[string], nil)
	runtime.GC()
	assert.Empty(t, s3)
}

//go:noinline
func map_gc() C_Map[C_String, int] {
	s := map[string]int{"abc": 1}
	return MapReprGoToC[string, int, C_String, int](s, StringReprGoToC[string], nil)
}
