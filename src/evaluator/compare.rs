use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use crate::models::{EndpointConfiguration, Grouping};

pub fn evaluate<'a>(
    openapi_endpoints: &'a Vec<EndpointConfiguration>,
    pre_merge_endpoints: &Option<Vec<EndpointConfiguration>>,
    nginx_endpoints: &Vec<EndpointConfiguration>,
    groupings: &HashSet<Grouping>,
) -> Evaluation<'a> {
    let mut grouping_endpoints: HashMap<&Grouping, Vec<RefCell<(&EndpointConfiguration, bool)>>> = HashMap::new();
    for grouping in groupings {
        grouping_endpoints.insert(grouping, vec![]);
    }

    let mut unmatched_endpoints: Vec<RefCell<(&EndpointConfiguration, bool)>> = vec![];
    let relevant_endpoints = get_endpoints_for_diff(pre_merge_endpoints, openapi_endpoints);

    for openapi_endpoint in &relevant_endpoints {
        let mut has_group = false;
        for grouping in grouping_endpoints.iter_mut() {
            if grouping.0.incompases_endpoint_config(openapi_endpoint) {
                has_group = true;
                if grouping.1.len() >= 1 && grouping.1[0].borrow().1 {
                    grouping.1.push(RefCell::new((openapi_endpoint, true)));
                } else {
                    if grouping.0.is_ignore_group {
                        grouping.1.push(RefCell::new((openapi_endpoint, true)));
                    } else if endpoint_incompases_any(openapi_endpoint, nginx_endpoints) {
                        for endpoint in grouping.1.iter_mut() {
                            let mut endpoint = endpoint.borrow_mut();
                            endpoint.1 = true;
                        }
                        grouping.1.push(RefCell::new((openapi_endpoint, true)));
                    } else {
                        add_endpoint_as_missed(openapi_endpoint, grouping.1, &mut unmatched_endpoints);
                    }
                }
            }
        }

        if !has_group {
            if !endpoint_incompases_any(openapi_endpoint, nginx_endpoints) {
                unmatched_endpoints.push(RefCell::new((openapi_endpoint, false)))
            }
        }
    }

    // filter for met endpoints
    unmatched_endpoints = unmatched_endpoints
        .iter()
        .map(|x| x.clone())
        .filter(|x| !x.borrow().1)
        .collect();

    let test_coverage = if relevant_endpoints.len() == 0 {
        1.0
    } else {
        (relevant_endpoints.len() as f32 - unmatched_endpoints.len() as f32) / relevant_endpoints.len() as f32
    };

    let has_gateway_issues = has_gateway_issues(nginx_endpoints);

    let endpoints_not_covered = unmatched_endpoints.iter().map(|x| x.borrow().0).collect();

    Evaluation {
        has_gateway_issues,
        test_coverage,
        endpoints_not_covered,
    }
}

fn endpoint_incompases_any(
    endpoint: &EndpointConfiguration,
    possibly_incompased_endpoints: &Vec<EndpointConfiguration>,
) -> bool {
    // possible optimisation: remove incompased endpoint configuration from list after finding it
    for possible_endpoint in possibly_incompased_endpoints {
        if endpoint.incompases_endpoint(possible_endpoint) {
            return true;
        }
    }
    false
}

fn get_endpoints_for_diff<'a>(
    pre_merge_endpoints: &Option<Vec<EndpointConfiguration>>,
    post_merge_endpoints: &'a Vec<EndpointConfiguration>,
) -> HashSet<&'a EndpointConfiguration> {
    let mut relevant_endpoints = HashSet::new();
    for post_endpoint in post_merge_endpoints {
        relevant_endpoints.insert(post_endpoint);
    }
    match pre_merge_endpoints {
        Some(pre_merge_endpoints) => {
            for pre_endpoint in pre_merge_endpoints {
                relevant_endpoints.take(pre_endpoint);
            }
        }
        _ => (),
    }
    relevant_endpoints
}

fn add_endpoint_as_missed<'a>(
    endpoint: &'a EndpointConfiguration,
    grouping_endpoints: &mut Vec<RefCell<(&'a EndpointConfiguration, bool)>>,
    unmatched_endpoints: &mut Vec<RefCell<(&'a EndpointConfiguration, bool)>>,
) {
    let endpoint = RefCell::new((endpoint, false));
    grouping_endpoints.push(endpoint.clone());
    unmatched_endpoints.push(endpoint.clone());
}

fn has_gateway_issues(nginx_endpoints: &Vec<EndpointConfiguration>) -> bool {
    let gateway_issues = nginx_endpoints.iter().filter(|x| x.status_code == 502).count();
    gateway_issues > 40 || gateway_issues > nginx_endpoints.len() / 4
}

pub struct Evaluation<'a> {
    pub has_gateway_issues: bool,
    pub test_coverage: f32,
    pub endpoints_not_covered: Vec<&'a EndpointConfiguration>,
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, str::FromStr, sync::Arc};

    use float_eq::assert_float_eq;

    use crate::{
        models::{EndpointConfiguration, Grouping, Method, OpenapiPath},
        utils::test::create_mock_runtime,
    };

    use super::{endpoint_incompases_any, evaluate, has_gateway_issues};

    fn create_endpoint_a() -> EndpointConfiguration {
        EndpointConfiguration::new(Method::GET, "/a", 200, Arc::new(create_mock_runtime()), false).unwrap()
    }

    fn create_endpoint_b() -> EndpointConfiguration {
        EndpointConfiguration::new(Method::GET, "/b", 200, Arc::new(create_mock_runtime()), false).unwrap()
    }

    fn create_endpoint_c() -> EndpointConfiguration {
        EndpointConfiguration::new(Method::POST, "/c", 200, Arc::new(create_mock_runtime()), false).unwrap()
    }

    #[test]
    fn evaluate_covers_simplest_test_coverage_case() {
        let openapi_endpoints = vec![create_endpoint_a(), create_endpoint_b(), create_endpoint_c()];
        let nginx_endpoints = vec![create_endpoint_a(), create_endpoint_b()];

        let evaluation = evaluate(&openapi_endpoints, &None, &nginx_endpoints, &HashSet::new());

        assert_float_eq!(evaluation.test_coverage, 2.0 / 3.0, abs <= 0.001);
    }

    #[test]
    fn evaluate_gives_full_coverage_when_no_wanted_and_no_provided() {
        let openapi_endpoints = vec![];
        let nginx_endpoints = vec![];

        let evaluation = evaluate(&openapi_endpoints, &None, &nginx_endpoints, &HashSet::new());
        assert_float_eq!(evaluation.test_coverage, 1.0, abs <= 0.001);
    }

    #[test]
    fn evaluate_gives_zero_if_no_nginx_endpoint() {
        let openapi_endpoints = vec![create_endpoint_a()];
        let nginx_endpoints = vec![];

        let evaluation = evaluate(&openapi_endpoints, &None, &nginx_endpoints, &HashSet::new());
        assert_float_eq!(evaluation.test_coverage, 0.0, abs <= 0.001);
    }

    #[test]
    fn evaluate_groups_two_endpoints() {
        let openapi_endpoints = vec![create_endpoint_a(), create_endpoint_b(), create_endpoint_c()];
        let nginx_endpoints = vec![create_endpoint_a(), create_endpoint_b()];
        let grouping = Grouping::new(
            vec![Method::GET, Method::POST],
            vec![200],
            OpenapiPath::from_str("/{foo}").unwrap(),
            false,
        );
        let mut groupings = HashSet::new();
        groupings.insert(grouping);

        let evaluation = evaluate(&openapi_endpoints, &None, &nginx_endpoints, &groupings);

        assert_float_eq!(evaluation.test_coverage, 1.0, abs <= 0.001);
    }

    #[test]
    fn internal_incompases_all_check_matches_base_case() {
        let endpoint = create_endpoint_a();
        let possibly_incompased_endpoints = vec![create_endpoint_a()];
        assert!(endpoint_incompases_any(&endpoint, &possibly_incompased_endpoints))
    }

    #[test]
    fn internal_incompases_all_check_functions_for_sized_arrays() {
        let endpoint = create_endpoint_c();
        let possibly_incompased_endpoints = vec![create_endpoint_a(), create_endpoint_b()];
        assert!(!endpoint_incompases_any(&endpoint, &possibly_incompased_endpoints))
    }

    #[test]
    fn internal_incompases_all_check_returns_false_for_empty_possibilities() {
        let endpoint = create_endpoint_a();
        let possibly_incompased_endpoints = vec![];
        assert!(!endpoint_incompases_any(&endpoint, &possibly_incompased_endpoints))
    }

    #[test]
    fn correctly_asserts_gateway_issues() {
        let nginx_endpoints =
            vec![EndpointConfiguration::new(Method::GET, "/", 502, Arc::new(create_mock_runtime()), false).unwrap()];

        assert!(has_gateway_issues(&nginx_endpoints));
    }
}
