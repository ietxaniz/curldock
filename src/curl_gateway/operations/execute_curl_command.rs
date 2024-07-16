use crate::curl_gateway::errors::{CurlGatewayError, ErrorKind};
use crate::curl_gateway::models::{CurlCommand, CurlCommandResult};
use crate::curl_gateway::operations::{
    call_curl, generate_curl_command_result, get_command_args_and_environment_variables, store_data,
};

pub fn execute_curl_command(command: CurlCommand) -> Result<CurlCommandResult, CurlGatewayError> {
    let (args, env_vars) = get_command_args_and_environment_variables(&command).map_err(|e| {
        CurlGatewayError::with_source(
            ErrorKind::Execution,
            "execute_curl_command",
            "Failed to get command arguments and environment variables",
            Box::new(e),
        )
    })?;

    let (stdout, stderr) = call_curl(&args, &env_vars).map_err(|e| {
        CurlGatewayError::with_source(
            ErrorKind::Execution,
            "execute_curl_command",
            "Failed to execute curl command",
            Box::new(e),
        )
    })?;


    let result = generate_curl_command_result(command, &stdout, &stderr).map_err(|e| {
        CurlGatewayError::with_source(
            ErrorKind::DataHandling,
            "execute_curl_command",
            "Failed to generate curl command result",
            Box::new(e),
        )
    })?;

    store_data::store_data(result.clone().store_data).map_err(|e| {
        CurlGatewayError::with_source(
            ErrorKind::DataHandling,
            "execute_curl_command",
            "Failed to store curl command result",
            Box::new(e),
        )
    })?;

    Ok(result)
}
