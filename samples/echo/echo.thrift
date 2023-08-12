struct A {
    1: i32 number,
}

struct Ping {
    1: string msg,
    2: list<A> number_list,
    3: list<A> number_set,
}

struct Pong {
    1: string msg,
    2: map<i16,A> number_map,
}

service rustFFI {
    Pong echo_rs (1: Ping req),
}

service goFFI {
    Pong echo_go (1: Ping req),
}
