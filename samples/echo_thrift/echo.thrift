struct Ping {
    1: string msg,
}

struct Pong {
    1: string msg,
}

service rustFFI {
    Pong echo_rs (1: Ping req),
}

service goFFI {
    Pong echo_go (1: Ping req),
}
