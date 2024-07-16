pub mod generate_curl_command_01;
pub mod generate_curl_command_02;
pub mod get_command_args_and_environment_vars_02;
pub mod parse_curl_command_02;
pub mod call_curl_01;

pub mod calls;
pub use calls::check_curl_command_parse;
pub use calls::check_curl_command_result;