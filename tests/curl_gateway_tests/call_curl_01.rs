use curldock::curl_gateway::operations::{
    call_curl, get_command_args_and_environment_variables, parse_curl_command,
};
use std::fs;
use std::path::Path;

pub fn test() {
    let command_str = fs::read_to_string(Path::new("./test-assets/call_curl_01.sh"))
        .expect("Unable to read source file");

    let command = parse_curl_command(&command_str).expect("Unable to parse source file");

    let (args, vars) =
        get_command_args_and_environment_variables(&command).expect("error getting args and vars");

    let (stdout, stderr) = call_curl(&args, &vars).expect("error calling curl");

    fs::write(
        Path::new("./test-assets/tmp/call_curl_01.stdout"),
        stdout.clone(),
    )
    .expect("Unable to write stdout file");

    fs::write(
        Path::new("./test-assets/tmp/call_curl_01.stderr"),
        stderr.clone(),
    )
    .expect("Unable to write stderr file");
}
