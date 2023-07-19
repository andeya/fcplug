package gen

/*
   #cgo CFLAGS: -I/Users/henrylee2cn/rust/fcplug/target/debug
   #cgo LDFLAGS: -L/Users/henrylee2cn/rust/fcplug/target/debug -lffidl_demo

   #include "ffidl_demo.h"
*/
import "C"

import (
	"unsafe"

	"github.com/andeya/fcplug/go/ctypes"
)

var (
	_ unsafe.Pointer
	_ ctypes.C_DynArray[any]
)

type SearchRequest struct {
	Query         string `json:"query"`
	PageNumber    int32  `json:"page_number"`
	ResultPerPage int32  `json:"result_per_page"`
}
type G_SearchRequest struct {
	Query         ctypes.String `json:"query"`
	PageNumber    ctypes.Int32  `json:"page_number"`
	ResultPerPage ctypes.Int32  `json:"result_per_page"`
}

//go:inline
//go:nosplit
func (p *SearchRequest) ToObject() *G_SearchRequest {
	return (*G_SearchRequest)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_SearchRequest) ToScalar() *SearchRequest {
	return (*SearchRequest)(unsafe.Pointer(p))
}

type C_SearchRequest struct {
	Query         ctypes.C_String
	PageNumber    ctypes.Int32
	ResultPerPage ctypes.Int32
}

func (p C_SearchRequest) AsCgo() *C.struct_C_SearchRequest {
	return (*C.struct_C_SearchRequest)(unsafe.Pointer(&p))
}

var (
	_ ctypes.ReprGoToC[C_SearchRequest] = G_SearchRequest{}
	_ ctypes.ReprCToGo[G_SearchRequest] = C_SearchRequest{}
)

//go:inline
//go:nosplit
func (p G_SearchRequest) ToReprC(keepAliveRow *ctypes.KeepAliveRow) C_SearchRequest {
	return C_SearchRequest{
		Query:         p.Query.ToReprC(keepAliveRow),
		PageNumber:    p.PageNumber,
		ResultPerPage: p.ResultPerPage,
	}
}

//go:inline
//go:nosplit
func (p C_SearchRequest) ToReprGo() G_SearchRequest {
	return G_SearchRequest{
		Query:         p.Query.ToReprGo(),
		PageNumber:    p.PageNumber,
		ResultPerPage: p.ResultPerPage,
	}
}

type WebSite struct {
	Name   string            `json:"name"`
	Link   string            `json:"link"`
	Age    int32             `json:"age"`
	Server map[string]Server `json:"server"`
}
type G_WebSite struct {
	Name   ctypes.String                                                  `json:"name"`
	Link   ctypes.String                                                  `json:"link"`
	Age    ctypes.Int32                                                   `json:"age"`
	Server ctypes.Map[ctypes.String, G_Server, ctypes.C_String, C_Server] `json:"server"`
}

//go:inline
//go:nosplit
func (p *WebSite) ToObject() *G_WebSite {
	return (*G_WebSite)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_WebSite) ToScalar() *WebSite {
	return (*WebSite)(unsafe.Pointer(p))
}

type C_WebSite struct {
	Name   ctypes.C_String
	Link   ctypes.C_String
	Age    ctypes.Int32
	Server ctypes.C_Map[ctypes.C_String, C_Server, ctypes.String, G_Server]
}

func (p C_WebSite) AsCgo() *C.struct_C_WebSite {
	return (*C.struct_C_WebSite)(unsafe.Pointer(&p))
}

var (
	_ ctypes.ReprGoToC[C_WebSite] = G_WebSite{}
	_ ctypes.ReprCToGo[G_WebSite] = C_WebSite{}
)

//go:inline
//go:nosplit
func (p G_WebSite) ToReprC(keepAliveRow *ctypes.KeepAliveRow) C_WebSite {
	return C_WebSite{
		Name:   p.Name.ToReprC(keepAliveRow),
		Link:   p.Link.ToReprC(keepAliveRow),
		Age:    p.Age,
		Server: p.Server.ToReprC(keepAliveRow),
	}
}

//go:inline
//go:nosplit
func (p C_WebSite) ToReprGo() G_WebSite {
	return G_WebSite{
		Name:   p.Name.ToReprGo(),
		Link:   p.Link.ToReprGo(),
		Age:    p.Age,
		Server: p.Server.ToReprGo(),
	}
}

type Server struct {
	Hostname string `json:"hostname"`
	Port     int32  `json:"port"`
}
type G_Server struct {
	Hostname ctypes.String `json:"hostname"`
	Port     ctypes.Int32  `json:"port"`
}

//go:inline
//go:nosplit
func (p *Server) ToObject() *G_Server {
	return (*G_Server)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_Server) ToScalar() *Server {
	return (*Server)(unsafe.Pointer(p))
}

type C_Server struct {
	Hostname ctypes.C_String
	Port     ctypes.Int32
}

func (p C_Server) AsCgo() *C.struct_C_Server {
	return (*C.struct_C_Server)(unsafe.Pointer(&p))
}

var (
	_ ctypes.ReprGoToC[C_Server] = G_Server{}
	_ ctypes.ReprCToGo[G_Server] = C_Server{}
)

//go:inline
//go:nosplit
func (p G_Server) ToReprC(keepAliveRow *ctypes.KeepAliveRow) C_Server {
	return C_Server{
		Hostname: p.Hostname.ToReprC(keepAliveRow),
		Port:     p.Port,
	}
}

//go:inline
//go:nosplit
func (p C_Server) ToReprGo() G_Server {
	return G_Server{
		Hostname: p.Hostname.ToReprGo(),
		Port:     p.Port,
	}
}

type RustFfi interface {
	Search(req SearchRequest) RustFfi_Search
}

type ImplRustFfi struct{}

func (i ImplRustFfi) Search(req SearchRequest) RustFfi_Search {
	// TODO implement me
	panic("implement me")
}

var _ RustFfi = ImplRustFfi{}

var keepAliveTable_rustffi_search = ctypes.NewKeepAliveTable("rustffi_search")

type RustFfi_Search struct {
	Ret     *WebSite
	cRetPtr *C.struct_C_WebSite
}

func (p *RustFfi_Search) Free() {
	if p != nil && p.cRetPtr != nil {
	}
}
