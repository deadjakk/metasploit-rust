use crate::*;
use serde_json::Value;

/// As the name implies, this is used to wrap the parameters received from
/// API requests. This is done so as to allow for easier retrieval of arguments.
#[derive(Debug)]
pub struct MsfParamWrapper(Option<Value>);

pub trait GetArg {
    fn get_arg(&self, val: &str) -> Option<String>;
}

impl MsfParamWrapper{
    pub fn new(val:Value)->Self{
        MsfParamWrapper(Some(val))
    }
}

impl GetArg for MsfParamWrapper {
    fn get_arg(&self, val: &str) -> Option<String>{
        let param = match self{
            MsfParamWrapper(Some(v)) => {
                match v.get(val){
                    Some(y) => y,
                    None => {
                        log_error(
                            &format!("parameter not received by module :{}",
                                val));
                        return None;
                    }
                }
            }
            MsfParamWrapper(None) => {
                log_error(&format!("value not received by module :{}",val));
                return None;
            }
        };
        let param_str = match param.as_str(){
            Some(v) => Some(v.to_string()),
            None =>{
                log_error(
                    &format!("unable to parse param into a string:{}\n\
                    received param: {:#?}", val,param));
                return None;
            }
        };
        param_str
    }
}

#[derive(serde_derive::Deserialize,serde_derive::Serialize, Debug)]
#[serde(rename_all = "camelCase")] // since msf requires 'type', a reserved kw
pub struct ReportParam {
    pub Type: String,
    pub data: HashMap<String,String>,
}

#[derive(serde_derive::Deserialize,serde_derive::Serialize, Debug)]
pub struct ReportRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: ReportParam,
}

#[derive(serde_derive::Deserialize,serde_derive::Serialize, Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub id: String,
    pub params: Value,
}

fn report(
    kind: &str, 
    data: HashMap<String,String>
) {
    let report_request: ReportRequest = ReportRequest {
        jsonrpc: "2.0".to_string(),
        method: "report".to_string(),
        params: ReportParam {
            Type: kind.to_string(),
            data: data,
        }
    };

    let report_request_string = serde_json::to_string(&report_request)
        .expect("couldn't serialize report_request;");
    rpc_send(report_request_string.as_str());
}

pub fn report_service(
    host: &str, 
    port: &str, 
    proto: &str, 
    provided_map: Option<HashMap<String,String>>
) {
    let mut option_map: HashMap<String,String> = HashMap::new();
    option_map.insert("host".to_string(),host.to_string());
    option_map.insert("port".to_string(),port.to_string());
    option_map.insert("proto".to_string(),proto.to_string());
    match provided_map {
        Some(v) => option_map.extend(v),
        None => (),
    }
    
    report("service",option_map);
}

pub fn report_vuln(
    host: &str, 
    name: &str, 
    provided_map: Option<HashMap<String,String>>
) {
    let mut option_map: HashMap<String,String> = HashMap::new();
    option_map.insert("host".to_string(),host.to_string());
    option_map.insert("name".to_string(),name.to_string());
    match provided_map {
        Some(v) => option_map.extend(v),
        None => (),
    }
    
    report("vuln",option_map);
}

pub fn report_wrong_password(
    username: &str, 
    password: &str, 
    provided_map: Option<HashMap<String,String>>,
) {
    let mut option_map: HashMap<String,String> = HashMap::new();
    option_map.insert("username".to_string(),username.to_string());
    option_map.insert("password".to_string(),password.to_string());
    match provided_map {
        Some(v) => option_map.extend(v),
        None => (),
    }
    
    report("wrong_password",option_map);
}

pub fn report_correct_password(
    username: &str, 
    password: &str, 
    provided_map: Option<HashMap<String,String>>
) {
    let mut option_map: HashMap<String,String> = HashMap::new();
    option_map.insert("username".to_string(),username.to_string());
    option_map.insert("password".to_string(),password.to_string());
    match provided_map {
        Some(v) => option_map.extend(v),
        None => (),
    }
    
    report("correct_password",option_map);
}

pub fn report_credential_login(
    address: &str, 
    port: &str, 
    protocol: &str, 
    service_name: &str, 
    provided_map: Option<HashMap<String,String>>
) {
    let mut option_map: HashMap<String,String> = HashMap::new();
    option_map.insert("address".to_string(),address.to_string());
    option_map.insert("port".to_string(),port.to_string());
    option_map.insert("protocol".to_string(),protocol.to_string());
    option_map.insert("service_name".to_string(),service_name.to_string());
    match provided_map {
        Some(v) => option_map.extend(v),
        None => (),
    }
    
    report("credential_login",option_map);
}

pub fn report_host(
    host: &str, 
    provided_map: Option<HashMap<String,String>>
) {
    let mut option_map: HashMap<String,String> = HashMap::new();
    option_map.insert("host".to_string(),host.to_string());
    match provided_map {
        Some(v) => option_map.extend(v),
        None => (),
    }
    
    report("host",option_map);
}
