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

type A struct {
	UserId   int32        `json:"user_id"`
	UserName string       `json:"user_name"`
	IsMale   bool         `json:"is_male"`
	Extra    map[string]B `json:"extra"`
}
type G_A struct {
	UserId   ctypes.Int32                                         `json:"user_id"`
	UserName ctypes.String                                        `json:"user_name"`
	IsMale   ctypes.Bool                                          `json:"is_male"`
	Extra    ctypes.Map[ctypes.String, G_B, ctypes.C_String, C_B] `json:"extra"`
}

//go:inline
//go:nosplit
func (p *A) ToObject() *G_A {
	return (*G_A)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_A) ToScalar() *A {
	return (*A)(unsafe.Pointer(p))
}

type C_A struct {
	UserId   ctypes.Int32
	UserName ctypes.C_String
	IsMale   ctypes.Bool
	Extra    ctypes.C_Map[ctypes.C_String, C_B, ctypes.String, G_B]
}

func (p C_A) AsCgo() *C.struct_C_A {
	return (*C.struct_C_A)(unsafe.Pointer(&p))
}

var (
	_ ctypes.ReprGoToC[C_A] = G_A{}
	_ ctypes.ReprCToGo[G_A] = C_A{}
)

//go:inline
//go:nosplit
func (p G_A) ToReprC(keepAliveRow *ctypes.KeepAliveRow) C_A {
	return C_A{
		UserId:   p.UserId,
		UserName: p.UserName.ToReprC(keepAliveRow),
		IsMale:   p.IsMale,
		Extra:    p.Extra.ToReprC(keepAliveRow),
	}
}

//go:inline
//go:nosplit
func (p C_A) ToReprGo() G_A {
	return G_A{
		UserId:   p.UserId,
		UserName: p.UserName.ToReprGo(),
		IsMale:   p.IsMale,
		Extra:    p.Extra.ToReprGo(),
	}
}

type GetUserResponse struct {
	Users   []User                    `json:"users"`
	RespMap map[string]GetUserRequest `json:"resp_map"`
	Req     GetUserRequest            `json:"req"`
}
type G_GetUserResponse struct {
	Users   ctypes.Slice[G_User, C_User]                                                   `json:"users"`
	RespMap ctypes.Map[ctypes.String, G_GetUserRequest, ctypes.C_String, C_GetUserRequest] `json:"resp_map"`
	Req     G_GetUserRequest                                                               `json:"req"`
}

//go:inline
//go:nosplit
func (p *GetUserResponse) ToObject() *G_GetUserResponse {
	return (*G_GetUserResponse)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_GetUserResponse) ToScalar() *GetUserResponse {
	return (*GetUserResponse)(unsafe.Pointer(p))
}

type C_GetUserResponse struct {
	Users   ctypes.C_Slice[C_User, G_User]
	RespMap ctypes.C_Map[ctypes.C_String, C_GetUserRequest, ctypes.String, G_GetUserRequest]
	Req     C_GetUserRequest
}

func (p C_GetUserResponse) AsCgo() *C.struct_C_GetUserResponse {
	return (*C.struct_C_GetUserResponse)(unsafe.Pointer(&p))
}

var (
	_ ctypes.ReprGoToC[C_GetUserResponse] = G_GetUserResponse{}
	_ ctypes.ReprCToGo[G_GetUserResponse] = C_GetUserResponse{}
)

//go:inline
//go:nosplit
func (p G_GetUserResponse) ToReprC(keepAliveRow *ctypes.KeepAliveRow) C_GetUserResponse {
	return C_GetUserResponse{
		Users:   p.Users.ToReprC(keepAliveRow),
		RespMap: p.RespMap.ToReprC(keepAliveRow),
		Req:     p.Req.ToReprC(keepAliveRow),
	}
}

//go:inline
//go:nosplit
func (p C_GetUserResponse) ToReprGo() G_GetUserResponse {
	return G_GetUserResponse{
		Users:   p.Users.ToReprGo(),
		RespMap: p.RespMap.ToReprGo(),
		Req:     p.Req.ToReprGo(),
	}
}

type C struct {
	UserId int32 `json:"user_id"`
	IsMale bool  `json:"is_male"`
}
type G_C struct {
	UserId ctypes.Int32 `json:"user_id"`
	IsMale ctypes.Bool  `json:"is_male"`
}

//go:inline
//go:nosplit
func (p *C) ToObject() *G_C {
	return (*G_C)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_C) ToScalar() *C {
	return (*C)(unsafe.Pointer(p))
}

type C_C struct {
	UserId ctypes.Int32
	IsMale ctypes.Bool
}

func (p C_C) AsCgo() *C.struct_C_C {
	return (*C.struct_C_C)(unsafe.Pointer(&p))
}

var (
	_ ctypes.ReprGoToC[C_C] = G_C{}
	_ ctypes.ReprCToGo[G_C] = C_C{}
)

//go:inline
//go:nosplit
func (p G_C) ToReprC(_ *ctypes.KeepAliveRow) C_C {
	return *(*C_C)(unsafe.Pointer(&p))
}

//go:inline
//go:nosplit
func (p C_C) ToReprGo() G_C {
	return *(*G_C)(unsafe.Pointer(&p))
}

type GetUserRequest struct {
	UserId   int32  `json:"user_id"`
	UserName string `json:"user_name"`
	IsMale   bool   `json:"is_male"`
}
type G_GetUserRequest struct {
	UserId   ctypes.Int32  `json:"user_id"`
	UserName ctypes.String `json:"user_name"`
	IsMale   ctypes.Bool   `json:"is_male"`
}

//go:inline
//go:nosplit
func (p *GetUserRequest) ToObject() *G_GetUserRequest {
	return (*G_GetUserRequest)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_GetUserRequest) ToScalar() *GetUserRequest {
	return (*GetUserRequest)(unsafe.Pointer(p))
}

type C_GetUserRequest struct {
	UserId   ctypes.Int32
	UserName ctypes.C_String
	IsMale   ctypes.Bool
}

func (p C_GetUserRequest) AsCgo() *C.struct_C_GetUserRequest {
	return (*C.struct_C_GetUserRequest)(unsafe.Pointer(&p))
}

var (
	_ ctypes.ReprGoToC[C_GetUserRequest] = G_GetUserRequest{}
	_ ctypes.ReprCToGo[G_GetUserRequest] = C_GetUserRequest{}
)

//go:inline
//go:nosplit
func (p G_GetUserRequest) ToReprC(keepAliveRow *ctypes.KeepAliveRow) C_GetUserRequest {
	return C_GetUserRequest{
		UserId:   p.UserId,
		UserName: p.UserName.ToReprC(keepAliveRow),
		IsMale:   p.IsMale,
	}
}

//go:inline
//go:nosplit
func (p C_GetUserRequest) ToReprGo() G_GetUserRequest {
	return G_GetUserRequest{
		UserId:   p.UserId,
		UserName: p.UserName.ToReprGo(),
		IsMale:   p.IsMale,
	}
}

type B struct {
	UserId int32 `json:"user_id"`
	IsMale bool  `json:"is_male"`
	C      C     `json:"c"`
}
type G_B struct {
	UserId ctypes.Int32 `json:"user_id"`
	IsMale ctypes.Bool  `json:"is_male"`
	C      G_C          `json:"c"`
}

//go:inline
//go:nosplit
func (p *B) ToObject() *G_B {
	return (*G_B)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_B) ToScalar() *B {
	return (*B)(unsafe.Pointer(p))
}

type C_B struct {
	UserId ctypes.Int32
	IsMale ctypes.Bool
	C      C_C
}

func (p C_B) AsCgo() *C.struct_C_B {
	return (*C.struct_C_B)(unsafe.Pointer(&p))
}

var (
	_ ctypes.ReprGoToC[C_B] = G_B{}
	_ ctypes.ReprCToGo[G_B] = C_B{}
)

//go:inline
//go:nosplit
func (p G_B) ToReprC(_ *ctypes.KeepAliveRow) C_B {
	return *(*C_B)(unsafe.Pointer(&p))
}

//go:inline
//go:nosplit
func (p C_B) ToReprGo() G_B {
	return *(*G_B)(unsafe.Pointer(&p))
}

type RustFfi interface {
	GetUser(shuffle string) RustFfi_GetUser
	GetUser2() RustFfi_GetUser2
	Test4(shuffle bool) int8
	Test5(shuffle bool) B
}

type ImplRustFfi struct{}

var _ RustFfi = ImplRustFfi{}

var keepAliveTable_rustffi_get_user = ctypes.NewKeepAliveTable("rustffi_get_user")

type RustFfi_GetUser struct {
	Ret     *string
	cRetPtr *C.struct_C_string
}

func (p *RustFfi_GetUser) Free() {
	if p != nil && p.cRetPtr != nil {
		C.rustffi_get_user_free_ret(p.cRetPtr)
	}
}
func (ImplRustFfi) GetUser(shuffle string) RustFfi_GetUser {
	var _keepAliveRow_rustffi = keepAliveTable_rustffi_get_user.NewRow()
	var _cRetPtr_rustffi = C.rustffi_get_user(*shuffle.ToObject().ToReprC(_keepAliveRow_rustffi).AsCgo())
	return RustFfi_GetUser{
		Ret:     (*string)(unsafe.Pointer(ctypes.Ref((*ctypes.C_String)(unsafe.Pointer(_cRetPtr_rustffi)).ToReprGo()))),
		cRetPtr: _cRetPtr_rustffi,
	}
}

var keepAliveTable_rustffi_get_user2 = ctypes.NewKeepAliveTable("rustffi_get_user2")

type RustFfi_GetUser2 struct {
	Ret     *GetUserResponse
	cRetPtr *C.struct_C_GetUserResponse
}

func (p *RustFfi_GetUser2) Free() {
	if p != nil && p.cRetPtr != nil {
		C.rustffi_get_user2_free_ret(p.cRetPtr)
	}
}
func (ImplRustFfi) GetUser2() RustFfi_GetUser2 {

	var _cRetPtr_rustffi = C.rustffi_get_user2()
	return RustFfi_GetUser2{
		Ret:     (*GetUserResponse)(unsafe.Pointer(ctypes.Ref((*C_GetUserResponse)(unsafe.Pointer(_cRetPtr_rustffi)).ToReprGo()))),
		cRetPtr: _cRetPtr_rustffi,
	}
}
func (ImplRustFfi) Test4(shuffle bool) int8 {
	return int8((ctypes.Int8)(C.rustffi_test4(*(*C._Bool)(unsafe.Pointer(&shuffle)))))
}

func (ImplRustFfi) Test5(shuffle bool) B {
	return *(*B)(unsafe.Pointer(ctypes.Ref(C.rustffi_test5(*(*C._Bool)(unsafe.Pointer(&shuffle))))))
}

type User struct {
	UserId   int32             `json:"user_id"`
	UserName string            `json:"user_name"`
	IsMale   bool              `json:"is_male"`
	Pure     A                 `json:"pure"`
	Extra    map[string]string `json:"extra"`
}
type G_User struct {
	UserId   ctypes.Int32                                                               `json:"user_id"`
	UserName ctypes.String                                                              `json:"user_name"`
	IsMale   ctypes.Bool                                                                `json:"is_male"`
	Pure     G_A                                                                        `json:"pure"`
	Extra    ctypes.Map[ctypes.String, ctypes.String, ctypes.C_String, ctypes.C_String] `json:"extra"`
}

//go:inline
//go:nosplit
func (p *User) ToObject() *G_User {
	return (*G_User)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_User) ToScalar() *User {
	return (*User)(unsafe.Pointer(p))
}

type C_User struct {
	UserId   ctypes.Int32
	UserName ctypes.C_String
	IsMale   ctypes.Bool
	Pure     C_A
	Extra    ctypes.C_Map[ctypes.C_String, ctypes.C_String, ctypes.String, ctypes.String]
}

func (p C_User) AsCgo() *C.struct_C_User {
	return (*C.struct_C_User)(unsafe.Pointer(&p))
}

var (
	_ ctypes.ReprGoToC[C_User] = G_User{}
	_ ctypes.ReprCToGo[G_User] = C_User{}
)

//go:inline
//go:nosplit
func (p G_User) ToReprC(keepAliveRow *ctypes.KeepAliveRow) C_User {
	return C_User{
		UserId:   p.UserId,
		UserName: p.UserName.ToReprC(keepAliveRow),
		IsMale:   p.IsMale,
		Pure:     p.Pure.ToReprC(keepAliveRow),
		Extra:    p.Extra.ToReprC(keepAliveRow),
	}
}

//go:inline
//go:nosplit
func (p C_User) ToReprGo() G_User {
	return G_User{
		UserId:   p.UserId,
		UserName: p.UserName.ToReprGo(),
		IsMale:   p.IsMale,
		Pure:     p.Pure.ToReprGo(),
		Extra:    p.Extra.ToReprGo(),
	}
}
