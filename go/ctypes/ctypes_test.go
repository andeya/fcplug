package ctypes

import (
	"runtime"
	"testing"

	"github.com/stretchr/testify/assert"
)

//go:noinline
func TestVec(t *testing.T) {
	s := []string{"abc"}
	vec := vec_gc()
	runtime.GC()
	t.Log(s, vec)
	runtime.GC()
	s2 := vec.IntoVec()
	runtime.GC()
	t.Log(s, vec, s2)
	s3 := vec.IntoVec()
	runtime.GC()
	assert.Empty(t, s3)
}

//go:noinline
func vec_gc() C_DynArray[string] {
	s := []string{"abc"}
	return FromVec(s)
}

//go:noinline
func TestString(t *testing.T) {
	s := "abc"
	vec := string_gc()
	runtime.GC()
	t.Log(s, vec)
	runtime.GC()
	s2 := IntoString[string](&vec)
	runtime.GC()
	t.Log(s, vec, s2)
	s3 := IntoString[string](&vec)
	runtime.GC()
	assert.Empty(t, s3)
}

//go:noinline
func string_gc() C_DynArray[byte] {
	s := "abc"
	return FromString(s)
}

//go:noinline
func TestMap(t *testing.T) {
	s := map[string]int{"abc": 1}
	vec := map_gc()
	runtime.GC()
	t.Log(s, vec)
	runtime.GC()
	s2 := IntoMap2[string, int](&vec)
	runtime.GC()
	t.Log(s, vec, s2)
	s3 := IntoMap2[string, int](&vec)
	runtime.GC()
	assert.Empty(t, s3)
}

//go:noinline
func map_gc() C_Map[string, int] {
	s := map[string]int{"abc": 1}
	return FromMap2(s)
}
