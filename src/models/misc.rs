#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[allow(clippy::should_implement_trait)]
impl Method {
    /// Converts a string representation of an HTTP method to a corresponding `Method` enum variant.
    ///
    /// # Arguments
    ///
    /// * `method_str` - The string representation of the HTTP method.
    ///
    /// # Returns
    ///
    /// An `Option` containing the corresponding `Method` enum variant if the conversion is successful,
    /// or `None` if the string does not match any known HTTP methods.
    pub fn from_str(method_str: &str) -> Option<Method> {
        match method_str.to_uppercase().as_str() {
            "GET" => Some(Method::GET),
            "PUT" => Some(Method::PUT),
            "POST" => Some(Method::POST),
            "DELETE" => Some(Method::DELETE),
            "OPTIONS" => Some(Method::OPTIONS),
            "HEAD" => Some(Method::HEAD),
            "PATCH" => Some(Method::PATCH),
            "TRACE" => Some(Method::TRACE),
            &_ => None,
        }
    }
}
