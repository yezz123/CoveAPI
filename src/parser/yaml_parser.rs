use std::sync::Arc;

use linked_hash_map::LinkedHashMap;
use yaml_rust::{Yaml, YamlLoader};

use crate::{
    config::Runtime,
    models::{EndpointConfiguration, Method},
    parser::common::format_basepath,
    utils::Error,
};

pub fn parse_yaml_doc(yaml_string: &str, runtime: Arc<Runtime>) -> Result<Vec<EndpointConfiguration>, Error> {
    let spec = match YamlLoader::load_from_str(yaml_string) {
        Ok(spec) => spec,
        Err(_) => return Err(Error::InvalidParseSyntax),
    };

    let spec = &spec[0];

    let spec = match spec.as_hash() {
        Some(spec) => spec,
        None => return Err(Error::UnknownInternalError("yaml spec can't be serialized".to_string())),
    };

    let basepath = match spec.get(&Yaml::from_str("basePath")) {
        Some(basepath) => match basepath.as_str() {
            Some(basepath) => basepath,
            None => return Err(Error::InvalidBasePath),
        },
        None => "",
    };
    let basepath = format_basepath(basepath);

    let paths = match spec.get(&Yaml::from_str("paths")) {
        Some(paths) => match paths.as_hash() {
            Some(paths) => paths,
            None => return Err(Error::InvalidParseSyntax),
        },
        None => return Err(Error::InvalidParseSyntax),
    };

    let mut endpoints = vec![];

    for path_key in paths.keys() {
        // unwrap is fine here, as we can expect keys to be strings
        let path = format!("{}{}", basepath, path_key.as_str().unwrap());
        let methods = retrive_value_as_hash_map(paths, path_key)?;

        for method_key in methods.keys() {
            // unwrap is fine here, as we can expect keys to be strings
            let method = match Method::from_str(method_key.as_str().unwrap()) {
                Some(method) => method,
                None => return Err(Error::InvalidParseMethod(String::from(path_key.as_str().unwrap()))),
            };

            let method_infos = retrive_value_as_hash_map(methods, method_key)?;
            let statuses = retrive_value_as_hash_map(method_infos, &Yaml::from_str("responses"))?;
            if method_infos.get(&Yaml::from_str("security")).is_some() {
                endpoints.push(EndpointConfiguration::new(
                    method.clone(),
                    &path,
                    401,
                    runtime.clone(),
                    true,
                )?);
                endpoints.push(EndpointConfiguration::new(
                    method.clone(),
                    &path,
                    403,
                    runtime.clone(),
                    true,
                )?);
            }

            for status_key in statuses.keys() {
                let status_code = match status_key.as_str().unwrap().parse() {
                    Ok(status_code) => status_code,
                    Err(_) => return Err(Error::InvalidParseStatusCode(status_key.as_str().unwrap().to_string())),
                };
                endpoints.push(EndpointConfiguration::new(
                    method.clone(),
                    &path,
                    status_code,
                    runtime.clone(),
                    false,
                )?);
            }
        }
    }

    Ok(endpoints)
}

fn retrive_value_as_hash_map<'a>(
    parent: &'a LinkedHashMap<Yaml, Yaml>,
    key: &Yaml,
) -> Result<&'a LinkedHashMap<Yaml, Yaml>, Error> {
    match parent.get(key) {
        Some(child) => match child.as_hash() {
            Some(child) => Ok(child),
            None => Err(Error::InvalidParseSyntax),
        },
        None => Err(Error::UnknownInternalError("yaml spec parent should exist".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, sync::Arc};

    use crate::{
        models::{Method, OpenapiPath},
        parser::yaml_parser::parse_yaml_doc,
        utils::test::create_mock_runtime,
    };

    const YAML_STRING: &str = "
basePath: /
paths:
  /:
    get:
      security:
        - BasicAuth: []
      responses:
        \"400\":
          description: Bad Request
          schema:
            $ref: '#/definitions/util.ErrorMessage'
        \"200\":
          description: OK
          schema:
            $ref: '#/definitions/controller.BaseResponse'
    put:
      responses:
        \"418\":
          description: Im a teapot
          schema:
            $ref: '#/definitions/controller.IsValid'
  /test:
    get:
      responses:
        \"418\":
          description: Im a teapot
          schema:
            $ref: '#/definitions/controller.IsValid'
";

    #[test]
    fn finds_all_paths() {
        assert!(parse_yaml_doc(YAML_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.path == OpenapiPath::from_str("/").unwrap()));
        assert!(parse_yaml_doc(YAML_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.path == OpenapiPath::from_str("/test").unwrap()));
    }

    #[test]
    fn finds_all_methods() {
        assert!(parse_yaml_doc(YAML_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.method == Method::GET));
        assert!(parse_yaml_doc(YAML_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.method == Method::PUT));
    }

    #[test]
    fn finds_all_statuses() {
        assert!(parse_yaml_doc(YAML_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.status_code == 200));
        assert!(parse_yaml_doc(YAML_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.status_code == 400));
        assert!(parse_yaml_doc(YAML_STRING, Arc::from(create_mock_runtime()))
            .unwrap()
            .iter()
            .any(|x| x.status_code == 418));
    }

    #[test]
    fn adds_401_403_for_security_headers() {
        assert_eq!(
            parse_yaml_doc(YAML_STRING, Arc::from(create_mock_runtime()))
                .unwrap()
                .iter()
                .filter(|x| x.method == Method::GET
                    && x.status_code == 401
                    && x.path == OpenapiPath::from_str("/").unwrap())
                .count(),
            1
        );
        assert_eq!(
            parse_yaml_doc(YAML_STRING, Arc::from(create_mock_runtime()))
                .unwrap()
                .iter()
                .filter(|x| x.method == Method::GET
                    && x.status_code == 403
                    && x.path == OpenapiPath::from_str("/").unwrap())
                .count(),
            1
        );
    }
}
