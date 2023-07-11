use std::process::{Command, Stdio};

use config::{configure_nginx, CoveAPIConfig};
use evaluator::evaluate;
use models::EndpointConfiguration;
use parser::{get_openapi_endpoint_configs, get_pre_merge_openapi_endpoints};
use utils::print_debug_message;

use crate::{parser::parse_nginx_access_log, utils::print_error_and_exit};

pub mod config;
pub mod evaluator;
pub mod models;
pub mod parser;
pub mod utils;

pub fn run_nginx(config: &CoveAPIConfig) {
    // insert application URL to nginx file
    match configure_nginx(config) {
        Ok(_) => (),
        Err(error) => error.display_error_and_exit(),
    }

    // spawn nginx as a subprocess
    print_debug_message("Starting nginx");
    let mut nginx_cmd = Command::new("nginx");
    nginx_cmd.arg("-g").arg("daemon off;");

    if !config.debug {
        nginx_cmd.stdout(Stdio::null());
    }

    match nginx_cmd.stdout(Stdio::null()).status() {
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

pub fn initialize_coveapi() -> (
    CoveAPIConfig,
    Vec<EndpointConfiguration>,
    Option<Vec<EndpointConfiguration>>,
) {
    let config = match CoveAPIConfig::from_env() {
        Ok(config) => config,
        Err(error) => error.display_error_and_exit(),
    };

    let openapi_endpoints = match get_openapi_endpoint_configs(&config) {
        Ok(openapi_endpoints) => openapi_endpoints,
        Err(error) => error.display_error_and_exit(),
    };

    let mut pre_merge_endpoints = None;

    // filter out impossible szenarios, where they require only_account_for_merge but nothing can
    // be compared
    if config.only_account_for_merge && !config.all_openapi_sources_are_paths() {
        if config.is_merge {
            print_error_and_exit("Your configuration contains a dynamically loaded openapi spec. CoveAPI needs it to be a local file when only accounting for the difference between commits.");
        } else {
            print_error_and_exit("You need to have two commits to compare (ex. pull/merge request) when only accounting for the difference between commits.");
        }
    }

    // add pre_merge_endpoints is a merge is taking place
    if config.is_merge && config.only_account_for_merge {
        let mut endpoints = vec![];

        for runtime in &config.runtimes {
            let mut pre_merge_endpoints_of_runtime = match get_pre_merge_openapi_endpoints(runtime.clone()) {
                Ok(endpoints) => endpoints,
                Err(err) => err.display_error_and_exit(),
            };
            endpoints.append(&mut pre_merge_endpoints_of_runtime);
        }
        pre_merge_endpoints = Some(endpoints);
    }
    (config, openapi_endpoints, pre_merge_endpoints)
}

pub fn run_eval(
    config: &CoveAPIConfig,
    openapi_endpoints: Vec<EndpointConfiguration>,
    pre_merge_endpoints: Option<Vec<EndpointConfiguration>>,
) {
    print_debug_message("Evaluating endpoint coverage");

    let nginx_endpoints = match parse_nginx_access_log(&config.runtimes) {
        Ok(nginx_endpoints) => nginx_endpoints,
        Err(_) => print_error_and_exit("An unexpected error occured while parsing the nginx logs"),
    };

    let evaluation = evaluate(
        &openapi_endpoints,
        &pre_merge_endpoints,
        &nginx_endpoints,
        &config.groupings,
    );

    if evaluation.has_gateway_issues {
        println!("WARNING: an unusual amount of 502 status codes were found, your setup might have gateway issues.");
    }

    println!("Test Coverage: {}%", evaluation.test_coverage * 100.0);

    if evaluation.endpoints_not_covered.len() > 0 {
        println!("The following endpoints were missed:");
        for endpoint in evaluation.endpoints_not_covered {
            println!("- {} {} {}", endpoint.path, endpoint.method, endpoint.status_code);
        }
    }
}
