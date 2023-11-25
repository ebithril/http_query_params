#[cfg(test)]
mod tests {
    use http_query_params::HttpQueryParams;
    use std::collections::HashMap;

    #[derive(HttpQueryParams, Default)]
    struct TestStruct {
        test_string: String,
        test_option_string: Option<String>,
        test_int: i32,
        test_option_int: Option<i32>,
    }

    #[test]
    fn test_string() {
        let test_struct = TestStruct {
            test_string: "test".to_string(),
            ..Default::default()
        };
        let params = test_struct.as_map();

        assert_eq!(params["test_string"], "test".to_string());
    }

    #[test]
    fn test_option_string() {
        let test_struct = TestStruct {
            test_option_string: Some("test".to_string()),
            ..Default::default()
        };
        let params = test_struct.as_map();

        assert_eq!(params["test_option_string"], "test".to_string());
    }

    #[test]
    fn test_int() {
        let test_struct = TestStruct {
            test_int: 1,
            ..Default::default()
        };
        let params = test_struct.as_map();

        assert_eq!(params["test_int"], 1.to_string());
    }

    #[test]
    fn test_option_int() {
        let test_struct = TestStruct {
            test_option_int: Some(1),
            ..Default::default()
        };
        let params = test_struct.as_map();

        assert_eq!(params["test_option_int"], 1.to_string());
    }

    #[test]
    fn test_none_option() {
        let test_struct = TestStruct::default();
        let params = test_struct.as_map();

        assert!(!params.contains_key("test_option"));
    }
}
