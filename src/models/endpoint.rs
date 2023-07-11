use super::misc::Method;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Endpoint {
    pub method: Method,
    pub path: String,
    pub status_code: u16,
}

impl Endpoint {
    /// Creates a new `Endpoint` instance.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method associated with the endpoint.
    /// * `path` - The path associated with the endpoint.
    /// * `status_code` - The status code associated with the endpoint.
    ///
    /// # Returns
    ///
    /// A new `Endpoint` instance with the specified method, path, and status code.
    pub fn new(method: Method, path: String, status_code: u16) -> Endpoint {
        Endpoint {
            method,
            path,
            status_code,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Method;

    use super::Endpoint;

    #[test]
    fn equality_checks_work() {
        let endpoint_a = Endpoint::new(Method::GET, String::from("/test"), 200);
        let endpoint_b = Endpoint::new(Method::GET, String::from("/test"), 200);

        assert!(endpoint_a == endpoint_b);
    }
}
