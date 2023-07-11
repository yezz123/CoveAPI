mod json_parser;
mod nginx_parser;

pub use json_parser::parse_openapi_json;
pub use nginx_parser::parse_nginx_access_log;

#[derive(Debug)]
pub enum ParsingError {
    InvalidSyntax,
    InvalidStatusCode,
    InvalidMethod,
    ProblemOpeningFile,
}
