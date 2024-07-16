
use curldock::curl_gateway::models::{CurlCommand, CurlOptions, HttpMethod, StoreCurlBody, StoreCurlCookie, LoadCurl};
use curldock::curl_gateway::operations::generate_curl_command;
use std::fs;
use std::path::Path;

pub fn test() {
    let test_dir = Path::new("./test-assets/tmp");
    let file_path = test_dir.join("generate_curl_command_02.sh");
    let file_name = file_path.to_str().unwrap().to_string();

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
        store_curl_body: vec![
          StoreCurlBody{
            source: "data.parameter1".to_string(),
            destination: "parameter1".to_string(),
            filename: "./test-assets/tmp/data.json".to_string(),
          }
        ],
        store_curl_cookie:vec![
          StoreCurlCookie{
            source: "auth".to_string(),
            destination: "auth".to_string(),
            filename: "./test-assets/tmp/data.json".to_string(),
          }
        ],
        load_curl: vec![
          LoadCurl{
            filename: "./test-assets/data.json".to_string(),
            data_name: "parameter1".to_string(),
            env_variable: "AUTH".to_string(),
          }
        ],
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

    let expected_file_path = Path::new("./test-assets/expected_generate_curl_command_02.sh");
    let expected_command =
        fs::read_to_string(expected_file_path).expect("Unable to read expected file");

    let generated_command = fs::read_to_string(file_name).expect("Unable to read generated file");

    assert_eq!(generated_command, expected_command);
}
