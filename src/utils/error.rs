use std::path::Path;

use super::print_error_and_exit;

#[derive(Debug)]
pub enum Error {
    InvalidApplicationURL(String),
    MissingConfiguration,
    ConflictingConfiguration,
    UnexpectedIOIssue(String),
    InvalidParseSyntax,
    InvalidBasePath,
    InvalidStatusCode(String),
    InvalidMethodString(String),
    InvalidParseStatusCode(String),
    InvalidParseMethod(String),
    ProblemOpeningFile(Box<Path>),
    UnknownInternalError(String),
    UnknownOpenApiFormat,
    InvalidTestCoverage,
    OpenapiFetchConnectionFailure,
    OpenapiFetchInvalidUrl,
    OpenapiMalformedOnlineComponents,
    InvalidPortNumber(String),
    InvalidMappingSyntax(String),
    MissingMapping,
    MappingMissingSemicolon(String),
    OpenapiPathIsAbsolute(Box<Path>),
    MappingDuplicatePorts,
    InvalidPath(String),
}

impl Error {
    fn get_error_msg(&self) -> String {
        match self {
            Error::InvalidApplicationURL(err_msg) => format!("Invalid application URL provided: {}", err_msg),
            Error::MissingConfiguration => "Your configuration is missing wither a mapping or an openapi source with it's respective application URL.".to_string(),
            Error::ConflictingConfiguration => "You can either provide a mapping or openapi location, port and application URL. Providing both is not possible at this time.".to_string(),
            Error::UnexpectedIOIssue(err_msg) => format!("An issue with IO occured: {}", err_msg),
            Error::ProblemOpeningFile(path) => format!("An issue opening the openapi ({:?}) file occured.", path),
            Error::InvalidParseSyntax => "The syntax of the openapi file is incorrect.".to_string(),
            Error::InvalidParseMethod(method) => format!("The openapi file contains an invalid method: {}", method),
            Error::InvalidParseStatusCode(code) => format!("The openapi file contains an invalid status code: {}", code),
            Error::UnknownInternalError(err) => format!("An unknown internal error occured, please open an issue on github for this [{}].", err),
            Error::InvalidBasePath => "Basepath provided in openapi spec isn't valid.".to_string(),
            Error::InvalidMethodString(method) => format!("The following method you provided is invalid: \"{}\"", method),
            Error::InvalidStatusCode(code) => format!("The following status code you provided is invalid: \"{}\"", code),
            Error::UnknownOpenApiFormat => "CoveAPI can only parse json and yaml formats,".to_string(),
            Error::InvalidTestCoverage => "Your test coverage has to be a value between 0 and 1 or a percentage between 0% and 100%.".to_string(),
            Error::OpenapiFetchConnectionFailure => "No connection to the specified openapi url could be made.".to_string(),
            Error::OpenapiFetchInvalidUrl => "The specified openapi url is invalid.".to_string(),
            Error::OpenapiMalformedOnlineComponents => "Some contents of the specified openapi resource are malformed.".to_string(),
            Error::InvalidPortNumber(port_str) => format!("The specified port number is invalid: \"{}\"", port_str),
            Error::InvalidMappingSyntax(mapping_string) => format!("The syntax of your mapping is invalid: {}", mapping_string),
            Error::MissingMapping => "Please provide a mapping to your configuration, the current mapping is either empty or wasn't provided.".to_string(),
            Error::MappingMissingSemicolon(mapping) => format!("The follwing mapping is missing a semicolon or is incomplete, please follow the 'service url; openapi source; port;' syntax: {}", mapping),
            Error::OpenapiPathIsAbsolute(path) => format!("The following path is absolute, please only specify relative paths: {}", path.to_str().unwrap_or("<empty>")),
            Error::MappingDuplicatePorts => "The mapping contains duplicate ports, every port can only be used once.".to_string(),
            Error::InvalidPath(path) => format!("The following path failed to parse: {}", path),
        }
    }

    pub fn display_error_and_exit(&self) -> ! {
        print!("Error: ");
        print_error_and_exit(self.get_error_msg())
    }

    pub fn display_error(&self) {
        eprintln!("{}", self.get_error_msg());
    }
}
