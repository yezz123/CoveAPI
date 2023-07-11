use std::path::Path;

use json::JsonValue;

use crate::{
    models::{Endpoint, Method},
    utils::read_file_to_string_or_err,
};

use super::ParsingError;

pub fn parse_openapi_json(path: &Path) -> Result<Vec<Endpoint>, ParsingError> {
    parse_json_doc(&read_file_to_string_or_err(path, ParsingError::ProblemOpeningFile)?)
}

fn parse_json_doc(json_string: &str) -> Result<Vec<Endpoint>, ParsingError> {
    let mut endpoints = vec![];

    let json_obj = match json::parse(json_string) {
        Ok(json_obj) => json_obj,
        Err(_) => return Err(ParsingError::InvalidSyntax),
    };

    let base_path = match &json_obj["basePath"] {
        JsonValue::Null => "",
        base_path => {
            if base_path == "/" {
                ""
            } else {
                println!("{}", base_path);
                match base_path.as_str() {
                    Some(base_path) => base_path,
                    None => return Err(ParsingError::InvalidSyntax),
                }
            }
        }
    };

    let paths = match &json_obj["paths"] {
        json::Null => return Err(ParsingError::InvalidSyntax),
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
                json::Null => return Err(ParsingError::InvalidSyntax),
                responses => responses,
            };

            for response in responses.entries() {
                let status_code = match response.0.parse() {
                    Ok(status_code) => status_code,
                    Err(_) => return Err(ParsingError::InvalidStatusCode),
                };
                endpoints.push(Endpoint::new(method.clone(), path.clone(), status_code))
            }
        }
    }

    Ok(endpoints)
}

fn get_methods_from_path(path_json: &JsonValue) -> Result<Vec<(Method, &JsonValue)>, ParsingError> {
    let mut methods = vec![];

    for method_entry in path_json.entries() {
        let method = match Method::from_str(method_entry.0) {
            Some(method) => method,
            None => return Err(ParsingError::InvalidMethod),
        };
        methods.push((method, method_entry.1));
    }
    Ok(methods)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::{
        models::Method,
        parser::json_parser::{parse_json_doc, parse_openapi_json},
    };

    const JSON_STRING: &str = r#"
    {
        "basePath": "/",
        "paths" : {
            "/": {
                "get": {
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
        assert_eq!(parse_json_doc(JSON_STRING).unwrap().len(), 4);
    }

    #[test]
    fn parses_correct_status_codes() {
        assert!(parse_json_doc(JSON_STRING)
            .unwrap()
            .iter()
            .any(|x| x.status_code == 200));
        assert!(parse_json_doc(JSON_STRING)
            .unwrap()
            .iter()
            .any(|x| x.status_code == 400));
        assert!(parse_json_doc(JSON_STRING)
            .unwrap()
            .iter()
            .any(|x| x.status_code == 418));
    }

    #[test]
    fn parses_correct_path() {
        assert!(parse_json_doc(JSON_STRING).unwrap().iter().any(|x| x.path == "/"));
        assert!(parse_json_doc(JSON_STRING).unwrap().iter().any(|x| x.path == "/test"));
    }

    #[test]
    fn parses_correct_method() {
        assert!(parse_json_doc(JSON_STRING)
            .unwrap()
            .iter()
            .any(|x| x.method == Method::GET));
        assert!(parse_json_doc(JSON_STRING)
            .unwrap()
            .iter()
            .any(|x| x.method == Method::POST));
        assert!(parse_json_doc(JSON_STRING)
            .unwrap()
            .iter()
            .any(|x| x.method == Method::PUT));
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
        println!("{:?}", parse_json_doc(JSON_STRING_DIFF_BASEPATH));
        assert!(parse_json_doc(JSON_STRING_DIFF_BASEPATH)
            .unwrap()
            .iter()
            .any(|x| x.path == "/foo"));
        assert!(parse_json_doc(JSON_STRING_DIFF_BASEPATH)
            .unwrap()
            .iter()
            .any(|x| x.path == "/foo/bar"));
    }

    #[test]
    fn parses_file_correctly() {
        let path = Path::new("./dump/swagger.json");
        assert_eq!(parse_openapi_json(&path).unwrap().len(), 4);
    }
}
