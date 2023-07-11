use CoveAPI::{initialize_coveapi, run_eval, run_nginx};

fn main() {
    let (config, openapi_endpoints) = initialize_coveapi();

    run_nginx(&config);

    run_eval(config, openapi_endpoints);
}
