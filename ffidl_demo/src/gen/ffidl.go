package gen

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
func (p *A) toObject() *G_A {
	return (*G_A)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_A) toScalar() *A {
	return (*A)(unsafe.Pointer(p))
}

type C_A struct {
	UserId   ctypes.Int32
	UserName ctypes.C_String
	IsMale   ctypes.Bool
	Extra    ctypes.C_Map[ctypes.C_String, C_B, ctypes.String, G_B]
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
func (p *GetUserRequest) toObject() *G_GetUserRequest {
	return (*G_GetUserRequest)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_GetUserRequest) toScalar() *GetUserRequest {
	return (*GetUserRequest)(unsafe.Pointer(p))
}

type C_GetUserRequest struct {
	UserId   ctypes.Int32
	UserName ctypes.C_String
	IsMale   ctypes.Bool
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
func (p *B) toObject() *G_B {
	return (*G_B)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_B) toScalar() *B {
	return (*B)(unsafe.Pointer(p))
}

type C_B = G_B

var (
	_ ctypes.ReprGoToC[C_B] = G_B{}
	_ ctypes.ReprCToGo[G_B] = C_B{}
)

//go:inline
//go:nosplit
func (p G_B) ToReprC(_ *ctypes.KeepAliveRow) C_B {
	return p
}

//go:inline
//go:nosplit
func (p C_B) ToReprGo() G_B {
	return p
}

type GetUserResponse struct {
	Users   []User                     `json:"users"`
	Resp    *GetUserResponse           `json:"resp"`
	RespMap map[string]GetUserResponse `json:"resp_map"`
	Req     GetUserRequest             `json:"req"`
}
type G_GetUserResponse struct {
	Users   ctypes.Slice[G_User, C_User]                                                     `json:"users"`
	Resp    *G_GetUserResponse                                                               `json:"resp"`
	RespMap ctypes.Map[ctypes.String, G_GetUserResponse, ctypes.C_String, C_GetUserResponse] `json:"resp_map"`
	Req     G_GetUserRequest                                                                 `json:"req"`
}

//go:inline
//go:nosplit
func (p *GetUserResponse) toObject() *G_GetUserResponse {
	return (*G_GetUserResponse)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_GetUserResponse) toScalar() *GetUserResponse {
	return (*GetUserResponse)(unsafe.Pointer(p))
}

type C_GetUserResponse struct {
	Users   ctypes.C_Slice[C_User, G_User]
	Resp    *C_GetUserResponse
	RespMap ctypes.C_Map[ctypes.C_String, C_GetUserResponse, ctypes.String, G_GetUserResponse]
	Req     C_GetUserRequest
}

var (
	_ ctypes.ReprGoToC[C_GetUserResponse] = G_GetUserResponse{}
	_ ctypes.ReprCToGo[G_GetUserResponse] = C_GetUserResponse{}
)

//go:inline
//go:nosplit
func (p G_GetUserResponse) ToReprC(keepAliveRow *ctypes.KeepAliveRow) C_GetUserResponse {
	return C_GetUserResponse{
		Users: p.Users.ToReprC(keepAliveRow),
		Resp: func(_Resp *G_GetUserResponse) *C_GetUserResponse {
			if _Resp != nil {
				return nil
			} else {
				_CResp := _Resp.ToReprC(keepAliveRow)
				return &_CResp
			}
		}(p.Resp),
		RespMap: p.RespMap.ToReprC(keepAliveRow),
		Req:     p.Req.ToReprC(keepAliveRow),
	}
}

//go:inline
//go:nosplit
func (p C_GetUserResponse) ToReprGo() G_GetUserResponse {
	return G_GetUserResponse{
		Users: p.Users.ToReprGo(),
		Resp: func(_Resp *C_GetUserResponse) *G_GetUserResponse {
			if _Resp != nil {
				return nil
			} else {
				_GResp := _Resp.ToReprGo()
				return &_GResp
			}
		}(p.Resp),
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
func (p *C) toObject() *G_C {
	return (*G_C)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_C) toScalar() *C {
	return (*C)(unsafe.Pointer(p))
}

type C_C = G_C

var (
	_ ctypes.ReprGoToC[C_C] = G_C{}
	_ ctypes.ReprCToGo[G_C] = C_C{}
)

//go:inline
//go:nosplit
func (p G_C) ToReprC(_ *ctypes.KeepAliveRow) C_C {
	return p
}

//go:inline
//go:nosplit
func (p C_C) ToReprGo() G_C {
	return p
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
func (p *User) toObject() *G_User {
	return (*G_User)(unsafe.Pointer(p))
}

//go:inline
//go:nosplit
func (p *G_User) toScalar() *User {
	return (*User)(unsafe.Pointer(p))
}

type C_User struct {
	UserId   ctypes.Int32
	UserName ctypes.C_String
	IsMale   ctypes.Bool
	Pure     C_A
	Extra    ctypes.C_Map[ctypes.C_String, ctypes.C_String, ctypes.String, ctypes.String]
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
