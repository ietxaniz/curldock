
use curldock::curl_gateway::models::{CurlCommand, CurlOptions, HttpMethod};
use curldock::curl_gateway::operations::generate_curl_command;
use std::fs;
use std::path::Path;

pub fn test() {
    // Define the test directory and file path
    let test_dir = Path::new("./test-assets/tmp");
    let file_path = test_dir.join("generate_curl_command_01.sh");
    let file_name = file_path.to_str().unwrap().to_string();

    // Create the test directory if it doesn't exist
    if !test_dir.exists() {
        fs::create_dir_all(test_dir).unwrap();
    }

    let options = CurlOptions {
        insecure: None,
        follow_redirects: None,
        max_redirects: None,
        timeout: None,
        connect_timeout: None,
        proxy: None,
        output_file: None,
        cert: None,
        key: None,
        key_password: None,
        compressed: None,
        retry: None,
        retry_delay: None,
        fail: None,
        interface: None,
        dns_servers: None,
        ipv4_only: None,
        ipv6_only: None,
        max_time: None,
        rate_limit: None,
    };

    let command = CurlCommand {
        url: "http://localhost:2080/".to_string(),
        method: HttpMethod::GET,
        headers: vec![],
        data: None,
        cookies: vec![],
        options: options,
        store_curl_body: vec![],
        store_curl_cookie: vec![],
        load_curl: vec![],
    };

    match generate_curl_command(&command) {
        Ok(curl_command) => {
            // Write the generated command to the file
            fs::write(&file_name, curl_command).expect("Unable to write file");
        }
        Err(e) => {
            panic!("Failed to generate curl command: {:?}", e);
        }
    }

    let expected_file_path = Path::new("./test-assets/expected_generate_curl_command_01.sh");
    let expected_command =
        fs::read_to_string(expected_file_path).expect("Unable to read expected file");

    let generated_command = fs::read_to_string(file_name).expect("Unable to read generated file");

    assert_eq!(generated_command, expected_command);
}
