package ctypes

import (
	"runtime"
	"testing"
	"unsafe"

	"github.com/stretchr/testify/assert"
)

var testKeepAliveTable = NewKeepAliveTable("test")

//go:noinline
func slice_gc(keepAliveRow *KeepAliveRow) C_Slice[C_String, String] {
	s := []string{"abc"}
	return (*(*Slice[String, C_String])(unsafe.Pointer(&s))).ToReprC(keepAliveRow)
}

//go:noinline
func TestSlice(t *testing.T) {
	var s = Slice[String, C_String]{"abc"}
	var keepAliveRow = testKeepAliveTable.NewRow()
	cslice := slice_gc(keepAliveRow)
	rowPtr := testKeepAliveTable.KeepRow(keepAliveRow)
	runtime.GC()
	t.Log(s, cslice)
	runtime.GC()
	s2 := cslice.ToReprGo()
	testKeepAliveTable.FreeRow(rowPtr)
	runtime.GC()
	assert.Equal(t, s, s2)
	t.Log(s, cslice, s2)
	s3 := cslice.ToReprGo()
	testKeepAliveTable.FreeRow(rowPtr)
	runtime.GC()
	assert.Equal(t, s, s3)
}

//go:noinline
func string_gc(keepAliveRow *KeepAliveRow) C_String {
	s := "abc"
	return String(s).ToReprC(keepAliveRow)
}

//go:noinline
func TestString(t *testing.T) {
	s := String("abc")
	var keepAliveRow = testKeepAliveTable.NewRow()
	cstring := string_gc(keepAliveRow)
	runtime.GC()
	rowPtr := testKeepAliveTable.KeepRow(keepAliveRow)
	runtime.GC()
	t.Log(s, cstring)
	s2 := cstring.ToReprGo()
	runtime.GC()
	testKeepAliveTable.FreeRow(rowPtr)
	runtime.GC()
	assert.Equal(t, s, s2)
	t.Log(s, cstring, s2)
	s3 := cstring.ToReprGo()
	runtime.GC()
	testKeepAliveTable.FreeRow(rowPtr)
	runtime.GC()
	assert.Equal(t, s, s3)
}

//go:noinline
func TestMap(t *testing.T) {
	s := Map[String, Int32, C_String, Int32]{"abc": 1}
	var keepAliveRow = testKeepAliveTable.NewRow()
	cmap := map_gc(keepAliveRow)
	runtime.GC()
	rowPtr := testKeepAliveTable.KeepRow(keepAliveRow)
	runtime.GC()
	t.Log(s, cmap)
	s2 := cmap.ToReprGo()
	runtime.GC()
	testKeepAliveTable.FreeRow(rowPtr)
	runtime.GC()
	assert.Equal(t, s, s2)
	t.Log(s, cmap, s2)
	s3 := cmap.ToReprGo()
	runtime.GC()
	testKeepAliveTable.FreeRow(rowPtr)
	runtime.GC()
	assert.Equal(t, s, s3)
}

//go:noinline
func map_gc(keepAliveRow *KeepAliveRow) C_Map[C_String, Int32, String, Int32] {
	s := map[string]int32{"abc": 1}
	return (*(*Map[String, Int32, C_String, Int32])(unsafe.Pointer(&s))).ToReprC(keepAliveRow)
}
