use std::{collections::HashMap, sync::Arc};

use crate::{config::Runtime, models::EndpointConfiguration};

pub fn sort_by_runtime(
    endpoint_configs: &Vec<EndpointConfiguration>,
) -> HashMap<Arc<Runtime>, Vec<&EndpointConfiguration>> {
    let mut runtime_sorted_endpoint_configs = HashMap::new();
    for endpoit_config in endpoint_configs {
        runtime_sorted_endpoint_configs
            .entry(endpoit_config.runtime.clone())
            .or_insert(vec![])
            .push(endpoit_config);
    }
    runtime_sorted_endpoint_configs
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        models::{EndpointConfiguration, Method},
        utils::test::create_mock_runtime,
    };

    use super::sort_by_runtime;

    #[test]
    fn sorts_into_runtime_correctly() {
        let mut runtime_a = create_mock_runtime();
        runtime_a.port = 400;
        let mut runtime_b = create_mock_runtime();
        runtime_b.port = 200;

        let runtime_a = Arc::from(runtime_a);
        let runtime_b = Arc::from(runtime_b);

        let endpoint_configs = vec![
            EndpointConfiguration::new(Method::GET, "/", 200, runtime_a.clone(), false).unwrap(),
            EndpointConfiguration::new(Method::GET, "/", 502, runtime_a.clone(), false).unwrap(),
            EndpointConfiguration::new(Method::GET, "/", 404, runtime_b.clone(), false).unwrap(),
        ];

        let sorted = sort_by_runtime(&endpoint_configs);

        assert_eq!(sorted.get(&runtime_a.clone()).unwrap().len(), 2);
        assert!(sorted
            .get(&runtime_a.clone())
            .unwrap()
            .iter()
            .any(|x| x.status_code == 200));
        assert!(sorted
            .get(&runtime_a.clone())
            .unwrap()
            .iter()
            .any(|x| x.status_code == 502));

        assert_eq!(sorted.get(&runtime_b.clone()).unwrap().len(), 1);
        assert!(sorted
            .get(&runtime_b.clone())
            .unwrap()
            .iter()
            .any(|x| x.status_code == 404));
    }
}
