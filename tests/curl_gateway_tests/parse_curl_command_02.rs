use super::calls::check_curl_command_parse;

pub fn test() {
    check_curl_command_parse(
        "./test-assets/expected_generate_curl_command_02.sh",
        "./test-assets/tmp/parse_curl_command_02.sh",
    );
}
