use std::sync::Arc;

use crate::{
    config::{OpenapiSource, Runtime},
    models::EndpointConfiguration,
    utils::{print_debug_message, Error},
};

use super::{json_parser::parse_json_doc, yaml_parser::parse_yaml_doc};

pub fn fetch_openapi_endpoints_for_runtime(runtime: Arc<Runtime>) -> Result<Vec<EndpointConfiguration>, Error> {
    let mut openapi_url = match &runtime.openapi_source {
        OpenapiSource::Url(openapi_url) => openapi_url.clone(),
        OpenapiSource::Path(_) => return Err(Error::UnknownInternalError("ota fetch with path".to_string())),
    };

    if openapi_url.host_str() == Some("localhost") {
        // unwrap here is fine, since the IP address provided is allways valid
        openapi_url.set_host(Some("172.17.0.1")).unwrap();
    }

    // note: using blocking client here because all following steps require it
    let openapi_spec = match reqwest::blocking::get(openapi_url.as_str()) {
        Ok(openapi_response) => match openapi_response.text() {
            Ok(openapi_spec) => openapi_spec,
            Err(why) => {
                print_debug_message(format!("{}", why));
                return Err(Error::OpenapiMalformedOnlineComponents);
            }
        },
        Err(why) => {
            print_debug_message(format!("{}", why));
            return Err(Error::OpenapiFetchConnectionFailure);
        }
    };

    // attempt to parse as json -> on syntax err attempt yaml
    match parse_json_doc(&openapi_spec, runtime.clone()) {
        Ok(endpoints) => Ok(endpoints),
        Err(Error::InvalidParseSyntax) => parse_yaml_doc(&openapi_spec, runtime.clone()),
        Err(error) => return Err(error),
    }
}
