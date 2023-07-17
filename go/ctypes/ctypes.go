package ctypes

// $ CGO_ENABLED=1 go build -buildmode=c-archive -o callee.a

import (
	"reflect"
	"runtime"
	"sync"
	"unsafe"
)

type KeepAliveTable struct {
	escapeGcMap sync.Map
	pool        sync.Pool
	name        string
}

func NewKeepAliveTable(name string) *KeepAliveTable {
	return &KeepAliveTable{name: name, pool: sync.Pool{New: func() any {
		return &KeepAliveRow{values: make([]any, 0, 8)}
	}}}
}

func (tab *KeepAliveTable) NewRow() *KeepAliveRow {
	return tab.pool.Get().(*KeepAliveRow)
}

type KeepAliveRow struct {
	values []any
}

func (row *KeepAliveRow) AddCell(value any) {
	row.values = append(row.values, value)
}

func (tab *KeepAliveTable) KeepRow(row *KeepAliveRow) uintptr {
	if len(row.values) > 0 {
		rowPtr := (*reflect.SliceHeader)(unsafe.Pointer(&row.values)).Data
		tab.escapeGcMap.Store(rowPtr, row)
		return rowPtr
	}
	tab.pool.Put(row)
	return 0
}

func (tab *KeepAliveTable) FreeRow(rowPtr uintptr) {
	if rowPtr > 0 {
		value, loaded := tab.escapeGcMap.LoadAndDelete(rowPtr)
		if loaded {
			row := value.(*KeepAliveRow)
			row.values = row.values[:0]
			tab.pool.Put(row)
		}
	}
}

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
func (c *C_DynArray[T]) consume() {
	if c != nil && c.Ptr > 0 {
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
		c.consume()
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
func SliceReprGoToC[G any, C any](keepAliveRow *KeepAliveRow, gslice []G, convElemFunc func(*KeepAliveRow, G) C) C_DynArray[C] {
	if len(gslice) == 0 {
		return C_DynArray[C]{}
	}
	if convElemFunc == nil {
		cslice := *(*C_DynArray[C])(unsafe.Pointer(&gslice))
		keepAliveRow.AddCell(gslice)
		runtime.KeepAlive(gslice)
		return cslice
	}
	cslice := make([]C, len(gslice))
	for i, t := range gslice {
		cslice[i] = convElemFunc(keepAliveRow, t)
	}
	vec := *(*C_DynArray[C])(unsafe.Pointer(&cslice))
	keepAliveRow.AddCell(cslice)
	runtime.KeepAlive(cslice)
	return vec
}

//go:inline
//go:nosplit
func SliceReprCToGo[C any, G any](cslice C_DynArray[C], convElemFunc func(C) G) []G {
	if cslice.IsEmpty() {
		return nil
	}
	if convElemFunc == nil {
		gslice := *(*[]G)(unsafe.Pointer(&cslice))
		return gslice
	}
	a := *(*[]C)(unsafe.Pointer(&cslice))
	gslice := make([]G, len(a))
	for i, t := range a {
		gslice[i] = convElemFunc(t)
	}
	return gslice
}

//go:inline
//go:nosplit
func BytesReprCToGo(cbytes C_Bytes) []byte {
	return SliceReprCToGo[byte, byte]((C_DynArray[byte])(cbytes), nil)
}

//go:inline
//go:nosplit
func BytesReprGoToC(keepAliveRow *KeepAliveRow, gbytes []byte) C_Bytes {
	return C_Bytes(SliceReprGoToC[byte, byte](keepAliveRow, gbytes, nil))
}

//go:inline
//go:nosplit
func StringReprCToGo[STRING ~string](cstr C_String) STRING {
	gslice := SliceReprCToGo[byte, byte]((C_DynArray[byte])(cstr), nil)
	if len(gslice) == 0 {
		return ""
	}
	return *(*STRING)(unsafe.Pointer(&gslice))
}

//go:inline
//go:nosplit
func StringReprGoToC[STRING ~string](keepAliveRow *KeepAliveRow, gstr STRING) C_String {
	if len(gstr) == 0 {
		return C_String{}
	}
	cslice := *(*C_DynArray[byte])(unsafe.Pointer(&struct {
		string
		Cap int
	}{*(*string)(unsafe.Pointer(&gstr)), len(gstr)}))
	keepAliveRow.AddCell(gstr)
	runtime.KeepAlive(gstr)
	return C_String(cslice)
}

type (
	ReprGoToC[C any] interface {
		ToReprC(keepAliveRow *KeepAliveRow) C
	}

	ReprCToGo[G any] interface {
		ToReprGo() G
	}

	KeyReprGoToC[CK any] interface {
		comparable
		ReprGoToC[CK]
	}
	KeyReprCToGo[GK any] interface {
		comparable
		ReprCToGo[GK]
	}
)

type (
	String   string
	C_String C_DynArray[byte]
)

var (
	_ ReprGoToC[C_String] = String("")
	_ ReprCToGo[String]   = C_String{}
)

//go:inline
//go:nosplit
func (p String) ToReprC(keepAliveRow *KeepAliveRow) C_String {
	return StringReprGoToC[String](keepAliveRow, p)
}

//go:inline
//go:nosplit
func (p C_String) ToReprGo() String {
	return StringReprCToGo[String](p)
}

type (
	Bytes   []byte
	C_Bytes C_DynArray[byte]
)

var (
	_ ReprGoToC[C_Bytes] = Bytes{}
	_ ReprCToGo[Bytes]   = C_Bytes{}
)

//go:inline
//go:nosplit
func (p Bytes) ToReprC(keepAliveRow *KeepAliveRow) C_Bytes {
	return BytesReprGoToC(keepAliveRow, p)
}

//go:inline
//go:nosplit
func (p C_Bytes) ToReprGo() Bytes {
	return BytesReprCToGo(p)
}

type (
	Slice[G ReprGoToC[C], C ReprCToGo[G]]   []G
	C_Slice[C ReprCToGo[G], G ReprGoToC[C]] C_DynArray[C]
)

var (
	_ ReprGoToC[C_Slice[Bool, Bool]] = Slice[Bool, Bool]{}
	_ ReprCToGo[Slice[Bool, Bool]]   = C_Slice[Bool, Bool]{}
)

//go:inline
//go:nosplit
func (c C_Slice[C, G]) ToReprGo() Slice[G, C] {
	return Slice[G, C](SliceReprCToGo[C, G](C_DynArray[C](c), nil))
}

//go:inline
//go:nosplit
func (s Slice[G, C]) ToReprC(keepAliveRow *KeepAliveRow) C_Slice[C, G] {
	return C_Slice[C, G](SliceReprGoToC[G, C](keepAliveRow, s, nil))
}

type (
	Map[GK KeyReprGoToC[CK], GV ReprGoToC[CV], CK KeyReprCToGo[GK], CV ReprCToGo[GV]]   map[GK]GV
	C_Map[CK KeyReprCToGo[GK], CV ReprCToGo[GV], GK KeyReprGoToC[CK], GV ReprGoToC[CV]] C_DynArray[MapEntry[CK, CV]]
	MapEntry[K comparable, V any]                                                       struct {
		Key   K
		Value V
	}
)

var (
	_ ReprGoToC[C_Map[C_String, C_String, String, String]] = Map[String, String, C_String, C_String]{}
	_ ReprCToGo[Map[String, String, C_String, C_String]]   = C_Map[C_String, C_String, String, String]{}
)

//go:inline
//go:nosplit
func (gmap Map[GK, GV, CK, CV]) ToReprC(keepAliveRow *KeepAliveRow) C_Map[CK, CV, GK, GV] {
	if len(gmap) == 0 {
		return C_Map[CK, CV, GK, GV]{}
	}
	cmap := make([]MapEntry[CK, CV], 0, len(gmap))
	for k, v := range gmap {
		cmap = append(cmap, MapEntry[CK, CV]{
			Key:   k.ToReprC(keepAliveRow),
			Value: v.ToReprC(keepAliveRow),
		})
	}
	return C_Map[CK, CV, GK, GV](SliceReprGoToC[MapEntry[CK, CV], MapEntry[CK, CV]](keepAliveRow, cmap, nil))
}

//go:inline
//go:nosplit
func (cmap C_Map[CK, CV, GK, GV]) ToReprGo() Map[GK, GV, CK, CV] {
	a := SliceReprCToGo[MapEntry[CK, CV], MapEntry[CK, CV]]((C_DynArray[MapEntry[CK, CV]])(cmap), nil)
	if len(a) == 0 {
		return nil
	}
	gmap := make(map[GK]GV, len(a))
	for _, entry := range a {
		gmap[entry.Key.ToReprGo()] = entry.Value.ToReprGo()
	}
	return gmap
}

//go:inline
//go:nosplit
func (cmap *C_Map[CK, CV, GK, GV]) Downcast() *C_DynArray[MapEntry[CK, CV]] {
	return (*C_DynArray[MapEntry[CK, CV]])(cmap)
}

// ===================== base type

var _ ReprGoToC[Bool] = Bool(false)
var _ ReprCToGo[Bool] = Bool(false)

var _ ReprGoToC[Int8] = Int8(0)
var _ ReprCToGo[Int8] = Int8(0)

var _ ReprGoToC[Int16] = Int16(0)
var _ ReprCToGo[Int16] = Int16(0)

var _ ReprGoToC[Int32] = Int32(0)
var _ ReprCToGo[Int32] = Int32(0)

var _ ReprGoToC[Int64] = Int64(0)
var _ ReprCToGo[Int64] = Int64(0)

var _ ReprGoToC[Uint8] = Uint8(0)
var _ ReprCToGo[Uint8] = Uint8(0)

var _ ReprGoToC[Uint16] = Uint16(0)
var _ ReprCToGo[Uint16] = Uint16(0)

var _ ReprGoToC[Uint32] = Uint32(0)
var _ ReprCToGo[Uint32] = Uint32(0)

var _ ReprGoToC[Uint64] = Uint64(0)
var _ ReprCToGo[Uint64] = Uint64(0)

var _ ReprGoToC[Float32] = Float32(0)
var _ ReprCToGo[Float32] = Float32(0)

var _ ReprGoToC[Float64] = Float64(0)
var _ ReprCToGo[Float64] = Float64(0)

var _ ReprGoToC[Void] = Void{}
var _ ReprCToGo[Void] = Void{}

type Bool bool

//go:inline
//go:nosplit
func (b Bool) ToReprGo() Bool {
	return b
}

//go:inline
//go:nosplit
func (b Bool) ToReprC(_ *KeepAliveRow) Bool {
	return b
}

type Int8 int8

//go:inline
//go:nosplit
func (i Int8) ToReprGo() Int8 {
	return i
}

//go:inline
//go:nosplit
func (i Int8) ToReprC(_ *KeepAliveRow) Int8 {
	return i
}

type Int16 int16

//go:inline
//go:nosplit
func (i Int16) ToReprGo() Int16 {
	return i
}

//go:inline
//go:nosplit
func (i Int16) ToReprC(_ *KeepAliveRow) Int16 {
	return i
}

type Int32 int32

//go:inline
//go:nosplit
func (i Int32) ToReprGo() Int32 {
	return i
}

//go:inline
//go:nosplit
func (i Int32) ToReprC(_ *KeepAliveRow) Int32 {
	return i
}

type Int64 int64

//go:inline
//go:nosplit
func (i Int64) ToReprGo() Int64 {
	return i
}

//go:inline
//go:nosplit
func (i Int64) ToReprC(_ *KeepAliveRow) Int64 {
	return i
}

type Uint8 uint8

//go:inline
//go:nosplit
func (u Uint8) ToReprGo() Uint8 {
	return u
}

//go:inline
//go:nosplit
func (u Uint8) ToReprC(_ *KeepAliveRow) Uint8 {
	return u
}

type Uint16 uint16

//go:inline
//go:nosplit
func (u Uint16) ToReprGo() Uint16 {
	return u
}

//go:inline
//go:nosplit
func (u Uint16) ToReprC(_ *KeepAliveRow) Uint16 {
	return u
}

type Uint32 uint32

//go:inline
//go:nosplit
func (u Uint32) ToReprGo() Uint32 {
	return u
}

//go:inline
//go:nosplit
func (u Uint32) ToReprC(_ *KeepAliveRow) Uint32 {
	return u
}

type Uint64 uint64

//go:inline
//go:nosplit
func (u Uint64) ToReprGo() Uint64 {
	return u
}

//go:inline
//go:nosplit
func (u Uint64) ToReprC(_ *KeepAliveRow) Uint64 {
	return u
}

type Float32 float32

//go:inline
//go:nosplit
func (f Float32) ToReprGo() Float32 {
	return f
}

//go:inline
//go:nosplit
func (f Float32) ToReprC(_ *KeepAliveRow) Float32 {
	return f
}

type Float64 float64

//go:inline
//go:nosplit
func (f Float64) ToReprGo() Float64 {
	return f
}

//go:inline
//go:nosplit
func (f Float64) ToReprC(_ *KeepAliveRow) Float64 {
	return f
}

type Void struct{}

//go:inline
//go:nosplit
func (v Void) ToReprGo() Void {
	return v
}

//go:inline
//go:nosplit
func (v Void) ToReprC(_ *KeepAliveRow) Void {
	return v
}
