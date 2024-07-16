use curldock::curl_gateway::operations::{
    get_command_args_and_environment_variables, parse_curl_command,
};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn test() {
    let command_str = fs::read_to_string(Path::new(
        "./test-assets/expected_generate_curl_command_02.sh",
    ))
    .expect("Unable to read source file");

    let command = parse_curl_command(&command_str).expect("Unable to parse source file");

    let (args, env_vars) = get_command_args_and_environment_variables(&command)
        .expect("could not get command args and environment variables");

    let args_path = Path::new("./test-assets/tmp/get_command_args_and_environment_vars_02.args");
    let mut args_file = File::create(&args_path).expect("Unable to create args file");
    for arg in args {
        writeln!(args_file, "{}", arg).expect("Unable to write to args file");
    }

    let env_vars_path =
        Path::new("./test-assets/tmp/get_command_args_and_environment_vars_02.env_vars");
    let mut env_vars_file = File::create(&env_vars_path).expect("Unable to create env vars file");
    for (key, value) in env_vars {
        writeln!(env_vars_file, "{}={}", key, value).expect("Unable to write to env vars file");
    }

    let expected_args = fs::read_to_string(Path::new(
        "./test-assets/expected_get_command_args_and_environment_vars_02.args",
    ))
    .expect("Unable to read source file");

    let created_args = fs::read_to_string(Path::new(
        "./test-assets/tmp/get_command_args_and_environment_vars_02.args",
    ))
    .expect("Unable to read source file");
    assert_eq!(expected_args, created_args);

    let expected_env_vars = fs::read_to_string(Path::new(
        "./test-assets/expected_get_command_args_and_environment_vars_02.env_vars",
    ))
    .expect("Unable to read source file");

    let created_env_vars = fs::read_to_string(Path::new(
        "./test-assets/tmp/get_command_args_and_environment_vars_02.env_vars",
    ))
    .expect("Unable to read source file");
    assert_eq!(expected_env_vars, created_env_vars);
}
