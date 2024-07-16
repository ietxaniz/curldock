use curldock::curl_gateway::operations::generate_curl_command;
use curldock::curl_gateway::operations::parse_curl_command;
use std::fs;
use std::path::Path;

pub fn check_curl_command_parse(input_file_name: &str, output_file_name: &str) {
    let command_str =
        fs::read_to_string(Path::new(&input_file_name)).expect("Unable to read source file");

    let command = parse_curl_command(&command_str).expect("Unable to parse source file");

    let result_command_str =
        generate_curl_command(&command).expect("Unable to generate source file");

    fs::write(Path::new(&output_file_name), result_command_str.clone())
        .expect("Unable to write file");

    assert_eq!(command_str, result_command_str);
}
