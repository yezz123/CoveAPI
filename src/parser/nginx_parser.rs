use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{
    models::{Endpoint, Method},
    parser::ParsingError,
};
use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_nginx_access_log() -> Result<Vec<Endpoint>, ParsingError> {
    parse_access_log(Path::new("/var/log/nginx/access.log"))
}

fn parse_access_log(path: &Path) -> Result<Vec<Endpoint>, ParsingError> {
    let mut endpoints = Vec::new();
    let reader = match File::open(path) {
        Ok(file) => BufReader::new(file),
        Err(_) => return Err(ParsingError::ProblemOpeningFile),
    };

    for line in reader.lines() {
        let line_str = match line {
            Ok(line_str) => line_str,
            Err(_) => return Err(ParsingError::ProblemOpeningFile),
        };

        endpoints.push(parse_nginx_line(&line_str)?);
    }

    Ok(endpoints)
}

fn parse_nginx_line(line: &str) -> Result<Endpoint, ParsingError> {
    lazy_static! {
        static ref NGINX_LINE_REGEX: Regex =
            Regex::new("^(\\[.+\\]) \"(\\w{3, 4}) (/\\S*) HTTP/\\d\\.\\d\" (\\d{3})").unwrap();
    }

    let captures = match NGINX_LINE_REGEX.captures(line) {
        Some(captures) => captures,
        None => return Err(ParsingError::InvalidSyntax),
    };

    let status = {
        let status_string = match captures.get(4) {
            Some(status_string) => status_string,
            None => return Err(ParsingError::InvalidSyntax),
        };

        match status_string.as_str().parse() {
            Ok(status) => status,
            Err(..) => return Err(ParsingError::InvalidStatusCode),
        }
    };

    let method = {
        let method_string = match captures.get(2) {
            Some(method_string) => method_string.as_str(),
            None => return Err(ParsingError::InvalidSyntax),
        };

        match Method::from_str(method_string) {
            Some(method) => method,
            None => return Err(ParsingError::InvalidMethod),
        }
    };

    let path = match captures.get(3) {
        Some(path) => String::from(path.as_str()),
        None => return Err(ParsingError::InvalidSyntax),
    };

    Ok(Endpoint::new(method, path, status))
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::{
        models::Method,
        parser::nginx_parser::{parse_access_log, parse_nginx_line},
    };

    #[test]
    fn parses_correct_status() {
        assert_eq!(
            parse_nginx_line("[11/Jul/2023:08:50:03 +0000] \"GET /weather HTTP/1.1\" 200")
                .unwrap()
                .status_code,
            200
        );
        assert_eq!(
            parse_nginx_line("[11/Jul/2023:08:52:45 +0000] \"GET /user HTTP/1.1\" 404")
                .unwrap()
                .status_code,
            404
        );
    }

    #[test]
    fn parses_correct_method() {
        assert_eq!(
            parse_nginx_line("[11/Jul/2023:08:50:03 +0000] \"GET /weather HTTP/1.1\" 200")
                .unwrap()
                .method,
            Method::GET
        );
        assert_eq!(
            parse_nginx_line("[11/Jul/2023:08:50:03 +0000] \"POST /weather HTTP/1.1\" 200")
                .unwrap()
                .method,
            Method::POST
        );
    }

    #[test]
    fn parses_correct_path() {
        assert_eq!(
            parse_nginx_line("[11/Jul/2023:08:50:03 +0000] \"GET /weather HTTP/1.1\" 200")
                .unwrap()
                .path,
            String::from("/weather")
        );
        assert_eq!(
            parse_nginx_line("[11/Jul/2023:08:52:45 +0000] \"GET /user HTTP/1.1\" 404")
                .unwrap()
                .path,
            String::from("/user")
        );
        assert_eq!(
            parse_nginx_line("[11/Jul/2023:08:52:45 +0000] \"GET / HTTP/1.1\" 404")
                .unwrap()
                .path,
            String::from("/")
        );
    }

    #[test]
    fn parses_full_access_log() {
        let path = Path::new("./dump/access.log");
        assert_eq!(parse_access_log(&path).unwrap().len(), 9);
    }
}
