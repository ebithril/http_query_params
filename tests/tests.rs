#[cfg(test)]
mod tests {
    use http_query_params::HttpQueryParams;
    use std::collections::HashMap;

    #[derive(HttpQueryParams, Default)]
    struct TestStruct {
        test_name: String,
        test_option: Option<String>,
    }

    #[test]
    fn test_string() {
        let test_struct = TestStruct {
            test_name: "test".to_string(),
            ..Default::default()
        };
        let params = test_struct.as_map();

        assert_eq!(params["test_name"], "test".to_string());
    }

    #[test]
    fn test_none_option() {
        let test_struct = TestStruct {
            ..Default::default()
        };
        let params = test_struct.as_map();

        assert!(!params.contains_key("test_option"));
    }

    #[test]
    fn test_option() {
        let test_struct = TestStruct {
            test_option: Some("test".to_string()),
            ..Default::default()
        };
        let params = test_struct.as_map();

        assert_eq!(params["test_option"], "test".to_string());
    }
}
