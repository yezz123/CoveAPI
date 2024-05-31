use std::sync::Arc;

use json::JsonValue;

use crate::{
    config::Runtime,
    models::{EndpointConfiguration, Method},
    utils::Error,
};

use super::common::format_basepath;

pub fn parse_json_doc(json_string: &str, runtime: Arc<Runtime>) -> Result<Vec<EndpointConfiguration>, Error> {
    let mut endpoints = vec![];

    let json_obj = match json::parse(json_string) {
        Ok(json_obj) => json_obj,
        Err(_) => return Err(Error::InvalidParseSyntax),
    };

    let base_path = match &json_obj["basePath"] {
        JsonValue::Null => "",
        base_path => match base_path.as_str() {
            Some(base_path) => base_path,
            None => return Err(Error::InvalidBasePath),
        },
    };
    let base_path = format_basepath(base_path);

    let paths = match &json_obj["paths"] {
        json::Null => return Err(Error::InvalidParseSyntax),
        responses => responses,
    };

    for path_json in paths.entries() {
        let mut path = String::from(base_path);

        match path_json.0 {
            "/" => (),
            _ => path.push_str(path_json.0),
        }
        if path.is_empty() {
            path.push('/');
        }

        for (method, method_json) in get_methods_from_path(path_json.1)?.into_iter() {
            let responses = match &method_json["responses"] {
                json::Null => return Err(Error::InvalidParseSyntax),
                responses => responses,
            };

            if !&method_json["security"].is_null() {
                endpoints.push(EndpointConfiguration::new(
                    method.clone(),
                    &path,
                    401,
                    runtime.clone(),
                    false,
                )?);
                endpoints.push(EndpointConfiguration::new(
                    method.clone(),
                    &path,
                    403,
                    runtime.clone(),
                    false,
                )?);
            }

            for response in responses.entries() {
                let status_code = match response.0.parse() {
                    Ok(status_code) => status_code,
                    Err(_) => return Err(Error::InvalidParseStatusCode(response.0.to_string())),
                };
                endpoints.push(EndpointConfiguration::new(
                    method.clone(),
                    &path,
                    status_code,
                    runtime.clone(),
                    false,
                )?)
            }
        }
    }

    Ok(endpoints)
}

fn get_methods_from_path(path_json: &JsonValue) -> Result<Vec<(Method, &JsonValue)>, Error> {
    let mut methods = vec![];

    for method_entry in path_json.entries() {
        let method = match Method::from_str(method_entry.0) {
            Some(method) => method,
            None => return Err(Error::InvalidParseMethod(method_entry.0.to_string())),
        };
        methods.push((method, method_entry.1));
    }
    Ok(methods)
}

#[cfg(test)]
mod test {

    use std::{str::FromStr, sync::Arc};

    use crate::{
        models::{Method, OpenapiPath},
        parser::json_parser::parse_json_doc,
        utils::test::create_mock_runtime,
    };

    const JSON_STRING: &str = r#"
    {
        "basePath": "/",
        "paths" : {
            "/": {
                "get": {
                    "security": [],
                    "responses": {
                        "200": {
                            "description": "OK",
                            "schema": {
                                "$ref": " #/definitions/controller.IsValid"
                            }
                        },
                        "400": {
                            "description": "Bad Request",
                            "schema": {
                                "$ref": " #/definitions/util.ErrorMessage"
                            }
                        }
                    }
                },
                "put": {
                    "responses": {
                        "418": {
                            "description": "I'm a teapot",
                            "schema": {
                                "$ref": " #/definitions/controller.IsValid"
                            }
                        }
                    }
                }
            },
            "/test": {
                "post": {
                    "responses": {
                        "418": {
                            "description": "I'm a teapot",
                            "schema": {
                                "$ref": " #/definitions/controller.IsValid"
                            }
                        }
                    }
                }
            }
        }
    }
    "#;

    #[test]
    fn parses_correct_number_of_responses() {
        assert_eq!(
            parse_json_doc(JSON_STRING, Arc::from(create_mock_runtime()))
                .unwrap()
                .len(),
            6
        );
    }

    #[test]
    fn parses_correct_status_codes() {
        assert!(parse_json_doc(JSON_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.status_code == 200));
        assert!(parse_json_doc(JSON_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.status_code == 400));
        assert!(parse_json_doc(JSON_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.status_code == 418));
    }

    #[test]
    fn parses_correct_path() {
        assert!(parse_json_doc(JSON_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.path == OpenapiPath::from_str("/").unwrap()));
        assert!(parse_json_doc(JSON_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.path == OpenapiPath::from_str("/test").unwrap()));
    }

    #[test]
    fn parses_correct_method() {
        assert!(parse_json_doc(JSON_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.method == Method::GET));
        assert!(parse_json_doc(JSON_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.method == Method::POST));
        assert!(parse_json_doc(JSON_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.method == Method::PUT));
    }

    #[test]
    fn adds_401_403_for_security_headers() {
        assert_eq!(
            parse_json_doc(JSON_STRING, Arc::from(create_mock_runtime()))
                .unwrap()
                .iter()
                .filter(|x| x.method == Method::GET
                    && x.status_code == 401
                    && x.path == OpenapiPath::from_str("/").unwrap())
                .count(),
            1
        );
        assert_eq!(
            parse_json_doc(JSON_STRING, Arc::from(create_mock_runtime()))
                .unwrap()
                .iter()
                .filter(|x| x.method == Method::GET
                    && x.status_code == 403
                    && x.path == OpenapiPath::from_str("/").unwrap())
                .count(),
            1
        );
    }

    const JSON_STRING_DIFF_BASEPATH: &str = r#"
    {
        "basePath": "/foo",
        "paths" : {
            "/": {
                "get": {
                    "responses": {
                        "200": {
                            "description": "OK",
                            "schema": {
                                "$ref": " #/definitions/controller.IsValid"
                            }
                        }
                    }
                }
            },
            "/bar": {
                "get": {
                    "responses": {
                        "200": {
                            "description": "OK",
                            "schema": {
                                "$ref": " #/definitions/controller.IsValid"
                            }
                        }
                    }
                }
            }
        }
    }
    "#;

    #[test]
    fn parses_correct_basepath() {
        assert!(
            parse_json_doc(JSON_STRING_DIFF_BASEPATH, Arc::from(create_mock_runtime()))
                .unwrap()
                .iter()
                .any(|x| x.path == OpenapiPath::from_str("/foo").unwrap())
        );
        assert!(
            parse_json_doc(JSON_STRING_DIFF_BASEPATH, Arc::from(create_mock_runtime()))
                .unwrap()
                .iter()
                .any(|x| x.path == OpenapiPath::from_str("/foo/bar").unwrap())
        );
    }
}
