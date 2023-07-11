use crate::models::Endpoint;

pub fn compare_endpoints(set_a: &Vec<Endpoint>, set_b: &Vec<Endpoint>) -> Vec<Endpoint> {
    let mut set_a = (*set_a).clone();
    let mut set_b = (*set_b).clone();
    set_a.sort();
    set_b.sort();

    filter_consecutive_duplicates(&mut set_a);
    filter_consecutive_duplicates(&mut set_b);

    let mut index_a = 0;

    while index_a < set_a.len() {
        let mut index_b = 0;
        let item_a = set_a.get(index_a).unwrap();

        let mut is_found = false;

        while index_b < set_b.len() && item_a >= set_b.get(index_b).unwrap() {
            let item_b = set_b.get(index_b).unwrap();

            if item_a == item_b {
                is_found = true;
                set_b.remove(index_b);
            } else {
                index_b += 1;
            }
        }

        if is_found {
            set_a.remove(index_a);
        } else {
            index_a += 1;
        }
    }

    set_a.append(&mut set_b);

    set_a
}

fn filter_consecutive_duplicates<T: PartialEq>(set: &mut Vec<T>) {
    if set.is_empty() {
        return;
    }

    let mut index = 0;

    while index < set.len() - 1 {
        let reduced_index = false;
        while set.get(index) == set.get(index + 1) {
            set.remove(index);
        }
        if !reduced_index {
            index += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        evaluator::compare::filter_consecutive_duplicates,
        models::{Endpoint, Method},
    };

    use super::compare_endpoints;

    fn create_endpoint_a() -> Endpoint {
        Endpoint::new(Method::GET, String::from("/"), 200)
    }

    fn create_endpoint_b() -> Endpoint {
        Endpoint::new(Method::POST, String::from("/"), 200)
    }

    fn create_endpoint_c() -> Endpoint {
        Endpoint::new(Method::POST, String::from("/99"), 200)
    }

    fn create_endpoint_d() -> Endpoint {
        Endpoint::new(Method::GET, String::from("/99"), 201)
    }

    #[test]
    fn flags_equal_sets_same_order_as_right() {
        let set_a = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        let set_b = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        assert_eq!(compare_endpoints(&set_a, &set_b).len(), 0);
    }

    #[test]
    fn flags_equal_sets_different_order_as_right() {
        let set_a = vec![
            create_endpoint_d(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_a(),
        ];

        let set_b = vec![
            create_endpoint_b(),
            create_endpoint_a(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        assert_eq!(compare_endpoints(&set_a, &set_b).len(), 0);
    }

    #[test]
    fn flags_different_lengths_as_incorrect() {
        let set_a = vec![create_endpoint_a(), create_endpoint_b(), create_endpoint_c()];

        let set_b = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        assert_ne!(compare_endpoints(&set_a, &set_b).len(), 0);
        assert_ne!(compare_endpoints(&set_b, &set_a).len(), 0);
    }

    #[test]
    fn flags_different_sets_as_incorrect() {
        let set_a = vec![create_endpoint_a(), create_endpoint_b(), create_endpoint_c()];

        let set_b = vec![create_endpoint_a(), create_endpoint_b(), create_endpoint_d()];

        assert_ne!(compare_endpoints(&set_a, &set_b).len(), 0);
        assert_ne!(compare_endpoints(&set_b, &set_a).len(), 0);
    }

    #[test]
    fn specifies_all_different_endpoints() {
        let set_a = vec![create_endpoint_a(), create_endpoint_b(), create_endpoint_c()];

        let set_b = vec![create_endpoint_a(), create_endpoint_b(), create_endpoint_d()];
        assert!(compare_endpoints(&set_a, &set_b)
            .iter()
            .any(|x| *x == create_endpoint_d()));
        assert!(compare_endpoints(&set_a, &set_b)
            .iter()
            .any(|x| *x == create_endpoint_c()));
    }

    #[test]
    fn diff_takes_duplicates_into_account() {
        let set_a = vec![
            create_endpoint_a(),
            create_endpoint_a(),
            create_endpoint_a(),
            create_endpoint_a(),
            create_endpoint_b(),
        ];

        let set_b = vec![create_endpoint_a(), create_endpoint_b()];
        assert_eq!(compare_endpoints(&set_a, &set_b).len(), 0);
        assert_eq!(compare_endpoints(&set_b, &set_a).len(), 0);
    }

    #[test]
    fn filter_leaves_regular_vecs_unchanged() {
        let mut set_a = vec![1, 2, 3, 4, 5];
        filter_consecutive_duplicates(&mut set_a);

        assert_eq!(set_a, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn filter_keeps_one_of_duplicates() {
        let mut set_a: Vec<u32> = vec![1, 1];
        filter_consecutive_duplicates(&mut set_a);

        assert_eq!(set_a, vec![1]);
    }

    #[test]
    fn filter_works_with_empty_list() {
        let mut set_a: Vec<u32> = vec![];
        filter_consecutive_duplicates(&mut set_a);

        assert_eq!(set_a, vec![] as Vec<u32>);
    }

    #[test]
    fn filters_consecutive_duplicates() {
        let mut set_a = vec![1, 2, 2, 3, 4, 5];
        filter_consecutive_duplicates(&mut set_a);

        assert_eq!(set_a, vec![1, 2, 3, 4, 5]);
    }
}
