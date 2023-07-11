use std::{path::Path, process::Command};

use config::{ConfigurationError, CoveAPIConfig};
use evaluator::compare_endpoints;
use models::Endpoint;
use parser::{parse_openapi_json, ParsingError};
use utils::print_debug_message;

use crate::{parser::parse_nginx_access_log, utils::print_error_and_exit};

pub mod config;
pub mod evaluator;
pub mod models;
pub mod parser;
pub mod utils;

pub fn run_nginx(config: &CoveAPIConfig) {
    // spawn nginx as a subprocess
    print_debug_message(config, "Starting nginx");
    let mut nginx_cmd = Command::new("nginx");
    nginx_cmd.arg("-g").arg("daemon off;");

    match nginx_cmd.status() {
        Ok(status) => {
            if !status.success() {
                print_error_and_exit("Error: Unexpected non-zero exit code from nginx");
            }
        }
        Err(err) => {
            print_error_and_exit(format!("Error: Running Nginx failed with: {}", err));
        }
    }
}

pub fn initialize_coveapi() -> (CoveAPIConfig, Option<Vec<Endpoint>>) {
    let config = match CoveAPIConfig::from_path(Path::new("./coveapi.toml")) {
        Ok(config) => config,
        Err(ConfigurationError::IssueOpeningFile) => {
            print_error_and_exit("An issue opening configuration file (\"coveapi.toml\") occurred")
        }
        Err(ConfigurationError::IllegalSyntax(err)) => {
            print_error_and_exit(format!("The configuration file syntax is invalid: {}", err))
        }
    };

    let openapi_endpoints = match parse_openapi_json(config.environment.openapi_path.as_ref()) {
        Ok(openapi_endpoints) => openapi_endpoints,
        Err(ParsingError::ProblemOpeningFile) => print_error_and_exit("An issue opening the openapi file occurred."),
        Err(ParsingError::InvalidSyntax) => print_error_and_exit("The syntax of the openapi file is incorrect."),
        Err(ParsingError::InvalidMethod) => print_error_and_exit("The openapi file contains an invalid method."),
        Err(ParsingError::InvalidStatusCode) => {
            print_error_and_exit("The openapi file contains an invalid status code.")
        }
    };

    (config, Some(openapi_endpoints))
}

pub fn run_eval(config: CoveAPIConfig, openapi_endpoints: Option<Vec<Endpoint>>) {
    print_debug_message(&config, "Evaluating endpoint coverage");

    // TODO replace with dynamic fetch of spec
    let openapi_endpoints = openapi_endpoints.unwrap();

    let nginx_endpoints = match parse_nginx_access_log() {
        Ok(nginx_endpoints) => nginx_endpoints,
        Err(_) => print_error_and_exit("An unexpected error occurred while parsing the nginx logs"),
    };

    let endpoint_diff = compare_endpoints(&nginx_endpoints, &openapi_endpoints);

    if !endpoint_diff.len() == 0 {
        print_error_and_exit("Not all endpoints were tested!");
    } else {
        println!("Coverage 100%");
    }
}
