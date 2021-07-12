#![allow(non_snake_case)]

use std::error::Error;
use std::io::{Write,Read};
use std::collections::HashMap;
pub use serde_json::Value;

mod logging;
pub use logging::*;

mod report;
pub use report::*;

/// Function Pointer for the metasploit 'run' action callback
type RunFn = fn(&MsfParamWrapper)->Result<Option<String>,Box<dyn Error>>;

/// Function Pointer for the metasploit 'check' action callback
type CheckFn = fn(&MsfParamWrapper)->Result<(),Box<dyn Error>>;

#[derive(serde_derive::Deserialize,serde_derive::Serialize, Debug)]
#[serde(rename_all = "camelCase")] // since msf requires 'type', a reserved kw
pub struct Reference {
    pub Type: String, // a reserved keyword
    pub Ref: String,  // a reserved keyword
}

#[derive(serde_derive::Deserialize,serde_derive::Serialize, Debug)]
#[serde(rename_all = "camelCase")] // since msf requires 'type', a reserved kw
pub struct ModuleOption {
    pub Type: String, // rust kw
    pub description: String,
    pub required: bool,
    pub default: String,
}

#[derive(serde_derive::Deserialize,serde_derive::Serialize, Debug)]
#[serde(rename_all = "camelCase")] // since msf requires 'type', a reserved kw
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub date: String,
    pub Type: String,
    pub privileged: bool,
    pub options: HashMap<String,ModuleOption>,
    pub references: Vec<Reference>,
    // not sure if this is fully supported by the MSF API
    // so I will leave this out for now.
    // pub capabilities: Vec<String>, 
}

#[derive(serde_derive::Deserialize,serde_derive::Serialize, Debug)]
struct RunResponse {
    jsonrpc: String,
    id: String,
    result: RunResult,
}

#[derive(serde_derive::Deserialize,serde_derive::Serialize, Debug)]
#[serde(rename_all = "camelCase")] // since msf requires 'return', a rust kw
struct RunResult {
    Message: String,
    Return: String, // rust kw
}

#[derive(serde_derive::Deserialize,serde_derive::Serialize, Debug)]
pub struct MetadataResponse {
    pub jsonrpc: String,
    pub id: String,
    pub result: Metadata,
}

#[derive(serde_derive::Deserialize,serde_derive::Serialize, Debug)]
struct CheckError {
    code: isize,
    message: String,
}

#[derive(serde_derive::Deserialize,serde_derive::Serialize, Debug)]
struct CheckErrorResponse {
    jsonrpc: String,
    id: String,
    error: CheckError,
}

fn send_check_unsupported(
    id: &str,
) -> Result<(),Box<dyn Error>> {
    let check_error_response = CheckErrorResponse {
        jsonrpc: "2.0".to_string(),
        id: id.to_string(),
        error: CheckError {
            code: -32601,
            message: "Soft checks are not supported".to_string(),
        }
    };

    let check_error_response_str= serde_json::to_string(&check_error_response)?;
    rpc_send(&check_error_response_str);
    Ok(())
}

pub fn send_metadata(
    request: Request, 
    metadata: Metadata
) -> Result<(),Box<dyn Error>> {
    let metadata_response = MetadataResponse {
        jsonrpc: request.jsonrpc,
        id: request.id,
        result: metadata,
    };
    
    let metadata_resp_string = serde_json::to_string(&metadata_response)?;
    rpc_send(&metadata_resp_string);

    Ok(())
}

fn rpc_send(string: &str){
    print!("{}",string);
    std::io::stdout().flush().ok().expect("couldn't flush stdout");
}

fn rpc_read(
) -> Result<Request,Box<dyn Error>> {
    let mut stdin_h = std::io::stdin();
    let mut buf = [0u8; 10000];
    let num = stdin_h.read(&mut buf)?;
    // read data from stdin
    let req_dec = std::str::from_utf8(&buf)?;
    let mut data = req_dec.to_string();
    data.truncate(num);
    // deserialize into json
    let request: Request= serde_json::from_str(&data)?;
    Ok(request)
}

// sends the result after completing the module run
fn send_complete(
    id: &str,
    result: Option<String>,
)-> Result<(),Box<dyn Error>> {
    let result_str = match result {
        Some(v) => v,
        None => "".to_string(),
    };
    let run_result = RunResult {
        Message: "Module complete".to_string(),
        Return: result_str,
    };
    let run_response = RunResponse {
        jsonrpc: "2.0".to_string(),
        id: id.to_string(),
        result: run_result,
    };
    let run_response_string = serde_json::to_string(&run_response)?;
    rpc_send(&run_response_string);
    Ok(())
}

pub fn init(
    metadata: Metadata,
    run_fn: RunFn, 
    check_fn: Option<CheckFn>,
) -> Result<(),Box<dyn Error>> {
    let request = rpc_read()?;

    match &request.method.as_str() {
        &"describe" => {
            send_metadata(request,metadata)?;
        }
        &"run" => {
            let msfparams: MsfParamWrapper = MsfParamWrapper::new(request.params);
            let response = run_fn(&msfparams)?;
            send_complete(&request.id, response)?;
        }
        &"soft_check" => {
            match check_fn {
                // will keep this commented out until I know for sure
                // this is supported by MSF API
                // Some(check) => check(&request.params)?,
                // None => {
                //     log_error("check unsupported");
                //     send_check_unsupported(&request.id)?;
                // }
                _ => send_check_unsupported(&request.id)?,
            }
        }
        _ => eprintln!("ERROR: unknown method received"),
    }

    Ok(())
}
