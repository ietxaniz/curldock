use crate::debug_err;
use anyhow::Result;
use crate::curl_gateway::models::{CurlCommand, CurlCommandResult};
use crate::curl_gateway::operations::{
    call_curl, generate_curl_command_result, get_command_args_and_environment_variables, store_data,
};

pub fn execute_curl_command(command: CurlCommand) -> Result<CurlCommandResult> {
    let (args, env_vars) = debug_err!(get_command_args_and_environment_variables(&command))?;

    let (stdout, stderr) = debug_err!(call_curl(&args, &env_vars))?;


    let result = debug_err!(generate_curl_command_result(command, &stdout, &stderr))?;

    debug_err!(store_data::store_data(result.clone().store_data))?;

    Ok(result)
}
