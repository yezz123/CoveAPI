use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    sync::Arc,
};

use crate::{
    config::Runtime,
    models::{EndpointConfiguration, Method},
    utils::{print_debug_message, Error},
};
use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_nginx_access_log(runtimes: &Vec<Arc<Runtime>>) -> Result<Vec<EndpointConfiguration>, Error> {
    parse_access_log(runtimes, Path::new("/var/log/nginx/access.log"))
}

fn parse_access_log(runtimes: &Vec<Arc<Runtime>>, path: &Path) -> Result<Vec<EndpointConfiguration>, Error> {
    let mut endpoints = Vec::new();
    let reader = match File::open(path) {
        Ok(file) => BufReader::new(file),
        Err(why) => {
            print_debug_message(why.to_string());
            return Err(Error::ProblemOpeningFile(Box::from(path)));
        }
    };

    for line in reader.lines() {
        let line_str = match line {
            Ok(line_str) => line_str,
            Err(why) => {
                print_debug_message(why.to_string());
                return Err(Error::ProblemOpeningFile(Box::from(path)));
            }
        };

        endpoints.push(parse_nginx_line(runtimes, &line_str)?);
    }

    Ok(endpoints)
}

fn parse_nginx_line(runtimes: &Vec<Arc<Runtime>>, line: &str) -> Result<EndpointConfiguration, Error> {
    lazy_static! {
        static ref NGINX_LINE_REGEX: Regex =
            Regex::new("^(\\[.+\\]) \"(\\w{3, 4}) (/\\S*) HTTP/\\d\\.\\d\" (\\d{3}) (\\d{1, 5})").unwrap();
    }

    let captures = match NGINX_LINE_REGEX.captures(line) {
        Some(captures) => captures,
        None => return Err(Error::InvalidParseSyntax),
    };

    let status = {
        let status_string = match captures.get(4) {
            Some(status_string) => status_string,
            None => return Err(Error::InvalidParseSyntax),
        };

        match status_string.as_str().parse() {
            Ok(status) => status,
            Err(..) => return Err(Error::InvalidParseStatusCode(status_string.as_str().to_string())),
        }
    };

    let method = {
        let method_string = match captures.get(2) {
            Some(method_string) => method_string.as_str(),
            None => return Err(Error::UnknownInternalError("no method nginx logs".to_string())),
        };

        match Method::from_str(method_string) {
            Some(method) => method,
            None => {
                return Err(Error::UnknownInternalError(format!(
                    "invalid method nginx {}",
                    method_string
                )))
            }
        }
    };

    let path = match captures.get(3) {
        Some(path) => String::from(path.as_str()),
        None => return Err(Error::UnknownInternalError("invalid path nginx logs".to_string())),
    };

    let port = match captures.get(5) {
        Some(port_string) => match port_string.as_str().parse() {
            Ok(port) => port,
            Err(_) => return Err(Error::UnknownInternalError("invalid port nginx logs".to_string())),
        },
        None => return Err(Error::UnknownInternalError("no port number nginx logs".to_string())),
    };

    EndpointConfiguration::new(method, &path, status, find_runtime_by_port(runtimes, port)?, false)
}

fn find_runtime_by_port(runtimes: &Vec<Arc<Runtime>>, port: u16) -> Result<Arc<Runtime>, Error> {
    for runtime in runtimes {
        if runtime.port == port {
            return Ok(runtime.clone());
        }
    }
    Err(Error::UnknownInternalError("unknown port in nginx logs".to_string()))
}

#[cfg(test)]
mod test {
    use std::{path::Path, str::FromStr, sync::Arc};

    use reqwest::Url;

    use crate::{
        config::{OpenapiSource, Runtime},
        models::{Method, OpenapiPath},
        parser::nginx_parser::{parse_access_log, parse_nginx_line},
    };

    use super::find_runtime_by_port;

    fn generate_runtimes() -> Vec<Arc<Runtime>> {
        vec![
            Arc::from(Runtime {
                port: 13750,
                openapi_source: OpenapiSource::Path(Box::from(Path::new("./dump"))),
                app_base_url: Url::from_str("http://example.con").unwrap(),
            }),
            Arc::from(Runtime {
                port: 8080,
                openapi_source: OpenapiSource::Path(Box::from(Path::new("./dump"))),
                app_base_url: Url::from_str("http://example.con").unwrap(),
            }),
        ]
    }

    #[test]
    fn parses_correct_status() {
        assert_eq!(
            parse_nginx_line(
                &generate_runtimes(),
                "[11/Jul/2023:08:50:03 +0000] \"GET /weather HTTP/1.1\" 200 8080"
            )
            .unwrap()
            .status_code,
            200
        );
        assert_eq!(
            parse_nginx_line(
                &generate_runtimes(),
                "[11/Jul/2023:08:52:45 +0000] \"GET /user HTTP/1.1\" 404 8080"
            )
            .unwrap()
            .status_code,
            404
        );
    }

    #[test]
    fn parses_correct_method() {
        assert_eq!(
            parse_nginx_line(
                &generate_runtimes(),
                "[11/Jul/2023:08:50:03 +0000] \"GET /weather HTTP/1.1\" 200 8080"
            )
            .unwrap()
            .method,
            Method::GET
        );
        assert_eq!(
            parse_nginx_line(
                &generate_runtimes(),
                "[11/Jul/2023:08:50:03 +0000] \"POST /weather HTTP/1.1\" 200 8080"
            )
            .unwrap()
            .method,
            Method::POST
        );
    }

    #[test]
    fn parses_correct_path() {
        assert_eq!(
            parse_nginx_line(
                &generate_runtimes(),
                "[11/Jul/2023:08:50:03 +0000] \"GET /weather HTTP/1.1\" 200 8080"
            )
            .unwrap()
            .path,
            OpenapiPath::from_str("/weather").unwrap(),
        );
        assert_eq!(
            parse_nginx_line(
                &generate_runtimes(),
                "[11/Jul/2023:08:52:45 +0000] \"GET /user HTTP/1.1\" 404 8080"
            )
            .unwrap()
            .path,
            OpenapiPath::from_str("/user").unwrap(),
        );
        assert_eq!(
            parse_nginx_line(
                &generate_runtimes(),
                "[11/Jul/2023:08:52:45 +0000] \"GET / HTTP/1.1\" 404 8080"
            )
            .unwrap()
            .path,
            OpenapiPath::from_str("/").unwrap(),
        );
    }

    #[test]
    fn parses_correct_port() {
        assert_eq!(
            parse_nginx_line(
                &generate_runtimes(),
                "[11/Jul/2023:08:50:03 +0000] \"GET /weather HTTP/1.1\" 200 8080"
            )
            .unwrap()
            .runtime
            .port,
            8080
        );
        assert_eq!(
            parse_nginx_line(
                &generate_runtimes(),
                "[11/Jul/2023:08:50:03 +0000] \"POST /weather HTTP/1.1\" 200 13750"
            )
            .unwrap()
            .runtime
            .port,
            13750
        );
    }

    #[test]
    fn parses_full_access_log() {
        let path = Path::new("./dump/access.log");
        assert_eq!(parse_access_log(&generate_runtimes(), path).unwrap().len(), 9);
    }

    #[test]
    fn finds_runtime_by_port() {
        let runtimes = vec![
            Arc::from(Runtime {
                port: 8080,
                openapi_source: OpenapiSource::Path(Box::from(Path::new("./dump"))),
                app_base_url: Url::from_str("http://example.con").unwrap(),
            }),
            Arc::from(Runtime {
                port: 7890,
                openapi_source: OpenapiSource::Path(Box::from(Path::new("./dump"))),
                app_base_url: Url::from_str("http://example.con").unwrap(),
            }),
            Arc::from(Runtime {
                port: 443,
                openapi_source: OpenapiSource::Path(Box::from(Path::new("./dump"))),
                app_base_url: Url::from_str("http://example.con").unwrap(),
            }),
        ];
        assert_eq!(find_runtime_by_port(&runtimes, 7890).unwrap(), runtimes[1]);
        assert_eq!(find_runtime_by_port(&runtimes, 443).unwrap(), runtimes[2]);
    }

    #[test]
    fn throws_error_if_port_is_not_from_runtime() {
        let runtimes = vec![];
        assert!(find_runtime_by_port(&runtimes, 7890).is_err());
    }
}
