//! This is a regular crate doc comment, but it also contains a partial
//! Cargo manifest.  Note the use of a *fenced* code block, and the
//! `cargo` "language".
//!
//! ```cargo
//! [dependencies]
//! metasploit-shim = "*"
//! ```

#![allow(non_snake_case)]

extern crate metasploit_shim;

use std::error::Error;
use std::collections::HashMap;
use metasploit_shim::*;

fn main() -> Result<(),Box<dyn Error>> {

    let mut module_options: HashMap<String,ModuleOption> = HashMap::new();

    module_options.insert(
        "USERNAME".to_string(), ModuleOption{ 
            Type: "string".to_string(),
            description: "username to be used for module".to_string(),
            required: true,
            default: "default_username".to_string(),
        });
    module_options.insert(
        "RPORT".to_string(), ModuleOption{ 
            Type: "string".to_string(),
            description: "remote port".to_string(),
            required: true,
            default: "1080".to_string(),
        }
        );

    let metadata: Metadata = Metadata  {
        name: "testname".to_string(),
        description: "test description".to_string(),
        authors: vec![
            "deadjakk".to_string(),
            "another-guy".to_string(),
            ],
        references: vec![
            Reference {
                Type: "URL".to_string(), 
                Ref: "https://shell.rip".to_string()
            }
            ],
        date: "2021-07-12".to_string(),
        Type: "single_host_login_scanner".to_string(),
        privileged: true,
        options: module_options,
    };

    init(metadata,run_module,None)?;

    Ok(())
}

fn run_module(
    params: &MsfParamWrapper
) -> Result<Option<String>,Box<dyn Error>> {
    log_info("test-info");
    log_debug("test-debug");
    log_good("test-good");
    log_warning("test-warning");
    log_error("test-error");

    let mut data: HashMap<String,String> = HashMap::new();
    data.insert("username".to_string(),"admin1444".to_string());
    report_credential_login("1.1.1.1","1080","tcp","socks5",Some(data));

    report_correct_password("admin1","password1",None);

    let rhost_arg = params
        .get_arg("rhost")
        .expect("string param couldn't be retrieved");

    log_good(&format!("rhost argument: {}",rhost_arg));

    Ok(None)
}

