use std::fmt::Display;

const METHOD_GET_STR: &str = "GET";
const METHOD_PUT_STR: &str = "PUT";
const METHOD_POST_STR: &str = "POST";
const METHOD_DELETE_STR: &str = "DELETE";
const METHOD_OPTIONS_STR: &str = "OPTIONS";
const METHOD_HEAD_STR: &str = "HEAD";
const METHOD_PATCH_STR: &str = "PATCH";
const METHOD_TRACE_STR: &str = "TRACE";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Method {
    GET,
    PUT,
    POST,
    DELETE,
    OPTIONS,
    HEAD,
    PATCH,
    TRACE,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Method {
    pub fn as_str(&self) -> &str {
        match self {
            Method::GET => METHOD_GET_STR,
            Method::PUT => METHOD_PUT_STR,
            Method::POST => METHOD_POST_STR,
            Method::DELETE => METHOD_DELETE_STR,
            Method::OPTIONS => METHOD_OPTIONS_STR,
            Method::HEAD => METHOD_HEAD_STR,
            Method::PATCH => METHOD_PATCH_STR,
            Method::TRACE => METHOD_TRACE_STR,
        }
    }
}

#[allow(clippy::should_implement_trait)]
impl Method {
    pub fn from_str(method_str: &str) -> Option<Method> {
        match method_str.to_uppercase().as_str() {
            METHOD_GET_STR => Some(Method::GET),
            METHOD_PUT_STR => Some(Method::PUT),
            METHOD_POST_STR => Some(Method::POST),
            METHOD_DELETE_STR => Some(Method::DELETE),
            METHOD_OPTIONS_STR => Some(Method::OPTIONS),
            METHOD_HEAD_STR => Some(Method::HEAD),
            METHOD_PATCH_STR => Some(Method::PATCH),
            METHOD_TRACE_STR => Some(Method::TRACE),
            &_ => None,
        }
    }
}
