struct A {
    1: required i32 user_id,
    2: required string user_name,
    3: required bool is_male,
    10: optional map<string, B> extra,
}

struct B {
    1:  i32 user_id,
    2:  bool is_male,
    3: C c,
}

struct C {
    1:  i32 user_id,
    2:  bool is_male,
}

struct User {
    1: required i32 user_id,
    2: required string user_name,
    3: required bool is_male,
    4: A pure,

    10: optional map<string, string> extra,
}

struct GetUserRequest {
    1: i32 user_id,
    2: string user_name,
    3: bool is_male,
}

struct getUserResponse {
    1: list<User> users,
    3: optional map<string,GetUserRequest> respMap,
    4: GetUserRequest req,
}

service rustFFI {
    string GetUser (2: string shuffle),
    getUserResponse GetUser2 (),
    i8 test4 (1: bool shuffle),
    B test5 (1: bool shuffle),
}
