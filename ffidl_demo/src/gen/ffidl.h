#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct C_DynArray_u8 {
  uint8_t *ptr;
  uintptr_t len;
  uintptr_t cap;
} C_DynArray_u8;

typedef struct C_DynArray_u8 C_String;

typedef struct C {
  int32_t user_id;
  bool is_male;
} C;

typedef struct B {
  int32_t user_id;
  bool is_male;
  struct C c;
} B;

typedef struct B C_B;

typedef struct MapEntry_C_String__C_String {
  C_String key;
  C_String value;
} MapEntry_C_String__C_String;

typedef struct C_DynArray_MapEntry_C_String__C_String {
  struct MapEntry_C_String__C_String *ptr;
  uintptr_t len;
  uintptr_t cap;
} C_DynArray_MapEntry_C_String__C_String;

typedef struct C_DynArray_MapEntry_C_String__C_String C_Map_C_String__C_String;

typedef struct C_User {
  int32_t user_id;
  C_String user_name;
  bool is_male;
  C_B pure;
  C_Map_C_String__C_String *extra;
} C_User;

typedef struct C_DynArray_C_User {
  struct C_User *ptr;
  uintptr_t len;
  uintptr_t cap;
} C_DynArray_C_User;

typedef struct MapEntry_C_String__C_GetUserResponse {
  C_String key;
  struct C_GetUserResponse value;
} MapEntry_C_String__C_GetUserResponse;

typedef struct C_DynArray_MapEntry_C_String__C_GetUserResponse {
  struct MapEntry_C_String__C_GetUserResponse *ptr;
  uintptr_t len;
  uintptr_t cap;
} C_DynArray_MapEntry_C_String__C_GetUserResponse;

typedef struct C_DynArray_MapEntry_C_String__C_GetUserResponse C_Map_C_String__C_GetUserResponse;

typedef struct C_GetUserRequest {
  int32_t user_id;
  C_String user_name;
  bool is_male;
} C_GetUserRequest;

typedef struct C_GetUserResponse {
  struct C_DynArray_C_User users;
  struct C_GetUserResponse *resp;
  C_Map_C_String__C_GetUserResponse *resp_map;
  struct C_GetUserRequest req;
} C_GetUserResponse;

struct C_GetUserResponse ffidl_types(struct C_GetUserRequest a, bool b);

struct C_GetUserResponse *rustffi_get_user(struct C_GetUserRequest req, bool shuffle);

void rustffi_get_user_free_ret(struct C_GetUserResponse *ret_ptr);

struct C_GetUserResponse *rustffi_get_user2(void);

void rustffi_get_user2_free_ret(struct C_GetUserResponse *ret_ptr);

int8_t rustffi_test4(bool shuffle);

C_B rustffi_test5(bool shuffle);

extern struct C_GetUserResponse *goffi_get_user(struct C_GetUserRequest *req, bool shuffle);

extern void goffi_get_user_free_ret(uintptr_t ret_ptr);

extern struct C_GetUserResponse *goffi_get_user2(struct C_GetUserRequest *req);

extern void goffi_get_user2_free_ret(uintptr_t ret_ptr);

extern struct C_GetUserResponse *goffi_get_user3(bool shuffle);

extern void goffi_get_user3_free_ret(uintptr_t ret_ptr);

extern int8_t goffi_test4(bool shuffle);

extern C_B goffi_test5(bool shuffle);
