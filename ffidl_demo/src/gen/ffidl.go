package gen

type A struct {
	UserId   int32        `json:"user_id"`
	UserName string       `json:"user_name"`
	IsMale   bool         `json:"is_male"`
	Extra    map[string]B `json:"extra"`
}
type GetUserRequest struct {
	UserId   int32  `json:"user_id"`
	UserName string `json:"user_name"`
	IsMale   bool   `json:"is_male"`
}
type B struct {
	UserId int32 `json:"user_id"`
	IsMale bool  `json:"is_male"`
	C      C     `json:"c"`
}
type GetUserResponse struct {
	Users   []User                     `json:"users"`
	Resp    *GetUserResponse           `json:"resp"`
	RespMap map[string]GetUserResponse `json:"resp_map"`
	Req     GetUserRequest             `json:"req"`
}
type C struct {
	UserId int32 `json:"user_id"`
	IsMale bool  `json:"is_male"`
}
type User struct {
	UserId   int32             `json:"user_id"`
	UserName string            `json:"user_name"`
	IsMale   bool              `json:"is_male"`
	Pure     A                 `json:"pure"`
	Extra    map[string]string `json:"extra"`
}
