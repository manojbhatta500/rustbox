use serde::{Serialize,Deserialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]

pub enum Command {
    Set {key : String, value : String},
    Get {key : String},
    Upload{filename: String, content : Vec<u8>},
    Download{filename: String, content : Vec<u8>}
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