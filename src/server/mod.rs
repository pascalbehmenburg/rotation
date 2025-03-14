// RPC-Type API:
// - 1 Type per Request
// - Request Batching
// - Server push
// - Streaming Responses? - for now only batched requests/responses

// Request Wrapper?

enum Request<T> {
    GET(T),
    SUBSCRIBE(T),
    POST(T),
    PUT(T),
    BATCH(Vec<Request<T>>),
}

// TODO: autogenerate
enum RequestType {
    LOGIN,
    REGISTER,
    LOGOUT,
}
