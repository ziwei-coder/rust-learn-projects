use crate::http::method::Method;

/*
    GET /user?id=10 HTTP/1.1\r\n
    HEADERS \r\n
    BODY
*/

pub struct Request<'a> {
    path: &'a str,
    query_string: &'a str,
    method: Method,
}
