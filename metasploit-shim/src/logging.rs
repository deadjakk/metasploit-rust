use crate::*;

// logging functions for metasploit
fn msf_log(
    message: &str, 
    level: &str
) {
    let log_request: LogRequest = LogRequest {
        jsonrpc: "2.0".to_string(),
        method: "message".to_string(),
        params: LogParam {
            level: level.to_string(),
            message: message.to_string(),
        }
    };

    let log_request_string = serde_json::to_string(&log_request)
        .expect("couldn't serialize request_string");
    rpc_send(log_request_string.as_str());
}

pub fn log_error(
    message: &str
) {
	msf_log(message, "error")
}

pub fn log_warning(
    message: &str
) {
	msf_log(message, "warning")
}

pub fn log_good(
    message: &str
) {
	msf_log(message, "good")
}

pub fn log_info(
    message: &str
) {
	msf_log(message, "info")
}

pub fn log_debug(
    message: &str
) {
	msf_log(message, "debug")
}

#[derive(serde_derive::Deserialize,serde_derive::Serialize, Debug)]
pub struct LogParam {
    pub level: String,
    pub message: String,
}

#[derive(serde_derive::Deserialize,serde_derive::Serialize, Debug)]
pub struct LogRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: LogParam,
}
