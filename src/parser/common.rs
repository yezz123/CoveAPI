pub fn format_basepath(basepath: &str) -> &str {
    if basepath.ends_with('/') {
        &basepath[0..basepath.len() - 1]
    } else {
        basepath
    }
}

#[cfg(test)]
mod tests {
    use super::format_basepath;

    #[test]
    fn coverts_slash_to_empty_string() {
        assert_eq!(format_basepath("/"), "");
    }

    #[test]
    fn removes_trailing_slash() {
        assert_eq!(format_basepath("/hello/"), "/hello");
    }

    #[test]
    fn ignores_emty_string() {
        assert_eq!(format_basepath(""), "");
    }
}
