use serde::{Serialize,Deserialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]

pub enum Command {
    Set {key : String, value : String},
    Get {key : String},
    Upload{filename: String, content : Vec<u8>},
    Download{filename: String}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct  SetCommandResponse{
    pub success : bool,
    pub message : String
}

pub fn convert_set_response_to_bytes(set_cmd : &SetCommandResponse)-> Vec<u8>{
    let command_string = serde_json::to_string(set_cmd).unwrap();
    command_string.as_bytes().to_vec()
}

pub fn convert_bytes_to_set_response(bytes : Vec<u8>)->SetCommandResponse{
    let command_string = String::from_utf8_lossy(&bytes[..]);
    let cmd : SetCommandResponse = serde_json::from_str(&command_string).unwrap();
    cmd
}


#[derive(Serialize, Deserialize, Debug)]
pub struct  GetCommandResponse{
    pub success : bool,
    pub message : String
}


pub fn convert_get_response_to_bytes(set_cmd : &GetCommandResponse)-> Vec<u8>{
    let command_string = serde_json::to_string(set_cmd).unwrap();
    command_string.as_bytes().to_vec()
}

pub fn convert_bytes_to_get_response(bytes : Vec<u8>)->GetCommandResponse{
    let command_string = String::from_utf8_lossy(&bytes[..]);
    let cmd : GetCommandResponse = serde_json::from_str(&command_string).unwrap();
    cmd
}



pub fn convert_command_to_bytes(cmd: &Command){
    let command_string = serde_json::to_string(cmd).unwrap();
    println!("command_string is : {:?}",command_string);
    println!("command  bytes is {:?}",command_string.as_bytes());
    conevert_bytes_to_command(command_string.as_bytes() );
}

pub fn conevert_bytes_to_command(data : &[u8])-> Command{
    let command_string = String::from_utf8_lossy(&data[..]);
    let command : Command = serde_json::from_str(&command_string).unwrap();

    println!("converted data is : {:?}",command);

    command
}



#[derive(Debug, Clone)]
pub struct DataVault {
    pub key: String,
    pub value: String,
}

// Shared static vector (protected by a Mutex for thread-safety)
lazy_static::lazy_static! {
    pub static ref DATA_VAULT: Mutex<Vec<DataVault>> = Mutex::new(Vec::new());
}