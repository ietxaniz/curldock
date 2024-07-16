mod curl_gateway_tests;

#[cfg(test)]
mod tests {
    use crate::curl_gateway_tests::check_curl_command_result;

    use super::curl_gateway_tests::{
        call_curl_01, check_curl_command_parse, generate_curl_command_01, generate_curl_command_02,
        get_command_args_and_environment_vars_02,
    };

    #[test]
    fn test_call_curl_01() {
        call_curl_01::test();
    }

    #[test]
    fn test_generate_curl_command_01() {
        generate_curl_command_01::test();
    }

    #[test]
    fn test_generate_curl_command_02() {
        generate_curl_command_02::test();
    }

    #[test]
    fn test_get_command_args_and_environment_vars_02() {
        get_command_args_and_environment_vars_02::test();
    }

    #[test]
    fn test_parse_curl_command_02() {
        check_curl_command_parse(
            "./test-assets/expected_generate_curl_command_02.sh",
            "./test-assets/tmp/parse_curl_command_02.sh",
        );
    }

    #[test]
    fn test_parse_curl_command_04() {
        check_curl_command_parse(
            "./test-assets/expected_generate_curl_command_04.sh",
            "./test-assets/tmp/parse_curl_command_04.sh",
        );
    }

    #[test]
    fn test_parse_curl_command_result_03() {
        check_curl_command_result(
            "./test-assets/expected_generate_curl_command_03.sh",
            "./test-assets/tmp/result_curl_command_03.json",
            "./test-assets/expected_result_curl_command_03.json",
        );
    }

    #[test]
    fn test_parse_curl_command_result_04() {
        check_curl_command_result(
            "./test-assets/expected_generate_curl_command_04.sh",
            "./test-assets/tmp/result_curl_command_04.json",
            "./test-assets/expected_result_curl_command_04.json",
        );
    }

    #[test]
    fn test_parse_curl_command_03() {
        check_curl_command_parse(
            "./test-assets/expected_generate_curl_command_03.sh",
            "./test-assets/tmp/parse_curl_command_03.sh",
        );
    }
}
