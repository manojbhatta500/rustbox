use std::io::{Read, Write};
use std::{ env, net:: TcpStream};

use rustbox::{model, utils};
use rustbox::model::Command;


fn main(){
    let mut  args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        utils::exit_program("please provide sufficent command for rustbox cli".to_string());
    }
    args.remove(0);
    if args[0].to_lowercase() != "rustbox-cli" {
                help_printer();
        utils::show_error_messages("rustbox-cli is missing".to_string());
    }

    // remove rustbox-cli 
    args.remove(0);

    
    match parse_command(&args){
        Ok(cmd)=>{
            match handle_command(cmd) {
                Ok(_)=>{

                }
                Err(e)=>{
                    utils::show_error_messages(e);
                }
            }
        }    
        Err(e)=>{
            utils::show_error_messages(e);
        }
    }

//    i am confused here how will i match here the commands can you suggest me 
// only this part only 
    
}


fn connect_to_server(ip_address: String,port: String)->Result<TcpStream, std::io::Error>{
    let  server_full_address =  format!("{}:{}",ip_address,port);
    let tcp_stream= TcpStream::connect(server_full_address)?;
    Ok(tcp_stream)
}


fn parse_command(commands: &Vec<String>)-> Result<Command, String> {
    if commands.is_empty(){
        return Err("No command provided".to_string());
    }

    let cmd = commands[0].to_ascii_uppercase();

    match cmd.as_str() {
        "HELP" => {
            help_printer();
            Err("".to_string()) // just print help, not a real command
        }
        "SET" => {
            if commands.len() < 3 {
                return Err("SET requires <key> and <value>".to_string());
            }
            Ok(Command::Set {
                key: commands[1].clone(),
                value: commands[2].clone(),
            })
        }

        "GET" => {
            if commands.len() < 2 {
                return Err("GET requires <key>".to_string());
            }
            Ok(Command::Get {
                key: commands[1].clone(),
            })
        }

        "UPLOAD" => {
            if commands.len() < 2 {
                return Err("UPLOAD requires <filename>".to_string());
            }
            let filename = &commands[1];
            Ok(Command::Upload {
                filename: filename.clone(),
                content: [12,1].to_vec(),
            })
        }

        "DOWNLOAD" => {
            if commands.len() < 2 {
                return Err("DOWNLOAD requires <filename>".to_string());
            }
            Ok(Command::Download {
                filename: commands[1].clone(),
            })
        }

        _ => Err(format!(
            "Invalid command '{}'. Must be one of HELP, SET, GET, UPLOAD, DOWNLOAD.",
            cmd
        )),
    }
}



fn help_printer() {
    utils::show_messages("Available commands:".to_string());
    utils::show_messages("SET key value      -- set key-value on server".to_string());
    utils::show_messages("GET key            -- get value from server".to_string());
    utils::show_messages("UPLOAD filename    -- upload file to server".to_string());
    utils::show_messages("DOWNLOAD filename  -- download file from server".to_string());
    utils::show_messages("\nEXAMPLES:".to_string());
    utils::show_messages("rustbox-cli SET username Manoj".to_string());
    utils::show_messages("rustbox-cli GET username".to_string());
}



fn handle_command(cmd : Command)->Result<(),String>{

    match cmd {
        Command::Set { key, value }=>{
            validate_set_command(key.clone(), value.clone())?;
            exectute_set_command(key, value)?;
            Ok(())
        }
        Command::Get { key }=>{
            validate_get_command(key.clone())?;
            execute_get_command(key.clone())?;
            Ok(())
        }

        Command::Upload { filename, content }=>{
            Ok(())
        }
        
        Command::Download { filename }=>{
            Ok(())
        }

    }
}

fn validate_get_command(key: String)-> Result<(), String>{

    if key.trim().is_empty(){
                return Err("Key cannot be empty".to_string());
    }

    if key.chars().next().unwrap().is_numeric(){
        return Err("Key cannot start with a number".to_string());
    }


    if !key.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Key can only contain letters, digits, or underscores".to_string());
    }

    if key.len() > 50 {
        return Err("Key is too long (max 50 characters)".to_string());
    }
    Ok(())
}

fn execute_get_command(key : String)->Result<(),String>{

    let mut  tcp_stream = connect_to_server("127.0.0.1".to_string(), "8080".to_string()).map_err(|e|e.to_string())?;

    let command = Command::Get { key: key };

    let command_json = serde_json::to_string(&command).map_err(|e|e.to_string())?;

    tcp_stream.write_all(command_json.as_bytes()).map_err(|e|e.to_string())?;

    println!("✅ Command sent to server successfully.");

    let mut buffer = [0; 1024];
    let n = tcp_stream.read(&mut buffer).map_err(|e| e.to_string())?;
    
    let _response = model::convert_bytes_to_get_response(buffer[..n].to_vec());

    println!("response from server : {:?}",_response);

    Ok(())
}

fn validate_set_command(key : String, value : String)-> Result<(),String>{
    // here we will check if
    if key.trim().is_empty(){
        return Err("Key cannot be empty".to_string());
    }

    if value.trim().is_empty(){
        return Err("value cannot be empty".to_string());
    }

    if key.chars().next().unwrap().is_numeric() {
        return Err("Key cannot start with a number".to_string());
    }

    if !key.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Key can only contain letters, digits, or underscores".to_string());
    }

     if key.len() > 50 {
        return Err("Key is too long (max 50 characters)".to_string());
    }

    if value.len() > 500 {
        return Err("Value is too long (max 500 characters)".to_string());
    }


    Ok(())
}


fn exectute_set_command(key : String, value : String)-> Result<(),String>{

    let mut  tcp_stream = connect_to_server("127.0.0.1".to_string(), "8080".to_string()).map_err(|e|e.to_string())?;

    let command = Command::Set { key: key, value : value };

    let command_json = serde_json::to_string(&command).map_err(|e|e.to_string())?;
    

    tcp_stream.write_all(command_json.as_bytes()).map_err(|e|e.to_string())?;


    println!("✅ Command sent to server successfully.");

    let mut buffer = [0; 1024];
    let n = tcp_stream.read(&mut buffer).map_err(|e| e.to_string())?;
    
    let _response = model::convert_bytes_to_set_response(buffer[..n].to_vec());

    println!("operation success ✅");

    Ok(())
}