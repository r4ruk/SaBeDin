use std::collections::HashMap;

// function handles different params arriving from GET request
// TODO outsource into an URI helper
pub fn handle_params(params: &str) -> HashMap<String, String> {
    let mut map_params: HashMap<String, String>  = HashMap::new();
    let param_vec:Vec<&str> = params.split('&').collect::<Vec<&str>>();
    for param in param_vec {
        if let Some((name, value)) = param.split_once('=') {
            map_params.entry(name.to_string()).or_insert(value.to_string());
        } else {
            println!("couldnt read name value params")
        }
    }
    return map_params
}