use CoveAPI::{initialize_coveapi, run_eval, run_nginx};

fn main() {
    let (config, openapi_endpoints, pre_merge_endpoints) = initialize_coveapi();

    if config.debug {
        config.print();
    }
    run_nginx(&config);

    run_eval(&config, openapi_endpoints, pre_merge_endpoints);
}
