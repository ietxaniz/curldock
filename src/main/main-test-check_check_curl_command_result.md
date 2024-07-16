mod curl_gateway;

use curl_gateway::operations::{
  call_curl, generate_curl_command_result, get_command_args_and_environment_variables,
  parse_curl_command,
};
use serde_json::json;
use std::fs;
use std::path::Path;

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

  let json_result = json!({
      "responseHeaders": result.response_headers,
      "statusCode": result.status_code,
      "body": result.body,
      "cookies": result.cookies,
      "contentType": result.content_type,
      "redirectCount": result.redirect_count,
      "effectiveUrl": result.effective_url,
      "storeData": result.store_data,
  });

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


fn main() {
  check_curl_command_result(
    "./test-assets/expected_generate_curl_command_02.sh",
    "./test-assets/tmp/result_curl_command_02.json",
    "./test-assets/expected_generate_curl_command_02.sh",
);
}
