mod common;
mod http;
mod json_parser;
mod nginx_parser;
mod yaml_parser;

use std::{path::Path, sync::Arc};

pub use nginx_parser::parse_nginx_access_log;

use crate::{
    config::{CoveAPIConfig, OpenapiSource, Runtime},
    models::EndpointConfiguration,
    utils::{read_file_to_string_or_err, Error},
};

use self::{http::fetch_openapi_endpoints_for_runtime, json_parser::parse_json_doc, yaml_parser::parse_yaml_doc};

const OPENAPI_MOUNT_POINT: &str = "/repo";
const PRE_MERGE_PATH_EXTENSION: &str = ".coveapi.old";

pub fn get_openapi_endpoint_configs(config: &CoveAPIConfig) -> Result<Vec<EndpointConfiguration>, Error> {
    let mut openapi_endpoints = vec![];
    for runtime in &config.runtimes {
        let mut endpoints = match get_runtime_openapi_endpoint_configs(runtime.clone()) {
            Ok(endpoints) => endpoints,
            Err(err) => return Err(err),
        };
        openapi_endpoints.append(&mut endpoints);
    }
    Ok(openapi_endpoints)
}

pub fn get_runtime_openapi_endpoint_configs(runtime: Arc<Runtime>) -> Result<Vec<EndpointConfiguration>, Error> {
    match runtime.openapi_source {
        OpenapiSource::Url(_) => fetch_openapi_endpoints_for_runtime(runtime),
        OpenapiSource::Path(_) => parse_openapi_file(runtime, OPENAPI_MOUNT_POINT, ""),
    }
}

pub fn get_pre_merge_openapi_endpoints(runtime: Arc<Runtime>) -> Result<Vec<EndpointConfiguration>, Error> {
    match runtime.openapi_source {
        OpenapiSource::Url(_) => fetch_openapi_endpoints_for_runtime(runtime),
        OpenapiSource::Path(_) => parse_openapi_file(runtime, OPENAPI_MOUNT_POINT, PRE_MERGE_PATH_EXTENSION),
    }
}

pub fn parse_openapi_file(
    runtime: Arc<Runtime>,
    mount_point: &str,
    path_extension: &str,
) -> Result<Vec<EndpointConfiguration>, Error> {
    let openapi_path = match &runtime.openapi_source {
        OpenapiSource::Path(path) => path,
        OpenapiSource::Url(_) => return Err(Error::UnknownInternalError("open api path read on url".to_string())),
    };

    let mut buf = openapi_path.clone().into_path_buf();
    let extension = match buf.extension() {
        Some(extension) => match extension.to_str() {
            Some(extension) => extension.to_string(),
            None => return Err(Error::UnknownOpenApiFormat),
        },
        None => return Err(Error::UnknownOpenApiFormat),
    };

    let full_extension = format!("{}{}", extension, path_extension);

    buf.set_extension(full_extension);

    let openapi_path = Path::new(mount_point).join(buf);

    if extension == "json" {
        Ok(parse_json_doc(
            &read_file_to_string_or_err(
                &openapi_path,
                Error::ProblemOpeningFile(Box::from(openapi_path.as_path())),
            )?,
            runtime,
        )?)
    } else if extension == "yaml" || extension == "yml" {
        Ok(parse_yaml_doc(
            &read_file_to_string_or_err(
                &openapi_path,
                Error::ProblemOpeningFile(Box::from(openapi_path.as_path())),
            )?,
            runtime,
        )?)
    } else {
        Err(Error::UnknownOpenApiFormat)
    }
}

#[cfg(test)]
mod tests {
    use std::{path::Path, sync::Arc};

    use crate::{
        config::OpenapiSource,
        parser::{parse_openapi_file, PRE_MERGE_PATH_EXTENSION},
        utils::test::create_mock_runtime,
    };

    #[test]
    fn parses_json_file_correctly() {
        let path = Path::new("./dump/swagger.json");
        let mut runtime = create_mock_runtime();
        runtime.openapi_source = OpenapiSource::Path(Box::from(path));
        assert_eq!(parse_openapi_file(Arc::from(runtime), "./", "").unwrap().len(), 6);
    }

    #[test]
    fn parses_yaml_file_correctly() {
        let path = Path::new("./dump/swagger.yaml");
        let mut runtime = create_mock_runtime();
        runtime.openapi_source = OpenapiSource::Path(Box::from(path));
        assert_eq!(parse_openapi_file(Arc::from(runtime), "./", "").unwrap().len(), 6);
    }

    #[test]
    fn throws_error_when_providing_absolute_path() {
        let path = Path::new("/test");
        let mut runtime = create_mock_runtime();
        runtime.openapi_source = OpenapiSource::Path(Box::from(path));
        assert!(parse_openapi_file(Arc::from(runtime), "./", "").is_err())
    }

    #[test]
    fn parses_old_file_correctly() {
        let path = Path::new("./dump/swagger.yaml");
        let mut runtime = create_mock_runtime();
        runtime.openapi_source = OpenapiSource::Path(Box::from(path));
        assert_eq!(
            parse_openapi_file(Arc::from(runtime), "./", PRE_MERGE_PATH_EXTENSION)
                .unwrap()
                .len(),
            6
        );

        let path = Path::new("./dump/swagger.json");
        let mut runtime = create_mock_runtime();
        runtime.openapi_source = OpenapiSource::Path(Box::from(path));
        assert_eq!(
            parse_openapi_file(Arc::from(runtime), "./", PRE_MERGE_PATH_EXTENSION)
                .unwrap()
                .len(),
            6
        );
    }
}
