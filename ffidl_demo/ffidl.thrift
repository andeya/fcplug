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
    2: required getUserResponse resp,
    3: optional map<string,getUserResponse> respMap,
    4: GetUserRequest req,
}

service goFFI {
    getUserResponse getUser (1: GetUserRequest req, 2: bool shuffle),
    getUserResponse getUser2 (1: GetUserRequest req),
    getUserResponse getUser3 (1: bool shuffle),
    i8 test4 (1: bool shuffle),
    B test5 (1: bool shuffle),
}

service rustFFI {
    getUserResponse GetUser (1: GetUserRequest req, 2: bool shuffle),
    getUserResponse GetUser2 (),
    i8 test4 (1: bool shuffle),
    B test5 (1: bool shuffle),
}
