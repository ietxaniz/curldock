use curldock::curl_gateway::operations::{
    call_curl, generate_curl_command_result, get_command_args_and_environment_variables,
    parse_curl_command, store_data
};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

pub fn check_curl_command_result(
    input_file_name: &str,
    output_file_name: &str,
    data_file_name: &str,
) {
    let command_str =
        fs::read_to_string(Path::new(&input_file_name)).expect("Unable to read source file");

    let command = parse_curl_command(&command_str).expect("Unable to parse source file");

    let (args, vars) =
        get_command_args_and_environment_variables(&command).expect("error getting args and vars");

    let (stdout, stderr) = call_curl(&args, &vars).expect("error calling curl");

    let result = generate_curl_command_result(command, &stdout, &stderr)
        .expect("error generating curl command result");

    // Remove date and content-length from result.response_headers before adding to json
    let mut filtered_headers: HashMap<String, String> = result.response_headers.clone();
    filtered_headers.remove("date");
    filtered_headers.remove("content-length");

    let json_result = json!({
        "responseHeaders": filtered_headers,
        "statusCode": result.status_code,
        "body": result.body,
        "cookies": result.cookies,
        "contentType": result.content_type,
        "redirectCount": result.redirect_count,
        "effectiveUrl": result.effective_url,
        "storeData": result.store_data,
    });

    store_data::store_data(result.store_data).expect("could not store data");

    fs::write(
        Path::new(output_file_name),
        serde_json::to_string_pretty(&json_result).expect("Failed to serialize JSON"),
    )
    .expect("Unable to write output file");

    let data_content =
        fs::read_to_string(Path::new(data_file_name)).expect("could not read data_content");
    let output_content =
        fs::read_to_string(Path::new(output_file_name)).expect("could not read output_content");

    assert_eq!(
        data_content.trim(),
        output_content.trim(),
        "Output does not match expected data.\nExpected:\n{}\n\nActual:\n{}",
        data_content,
        output_content
    );
}
