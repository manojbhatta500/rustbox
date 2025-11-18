use std::{io::{Read, Write}, net::TcpListener, sync::{Arc, Mutex}};

use crate::{model::{self, SetCommandResponse}, operation, utils};


pub fn start_tcp_server(port_no : i32)->Result<TcpListener, std::io::Error>{
    let full_address = format!("127.0.0.1:{}", port_no);
    let  tcp_listener_object = TcpListener::bind(full_address)?;
    Ok(tcp_listener_object)
}


pub fn handle_connection(tcp_object : &TcpListener){
    for stream in tcp_object.incoming(){
        match stream {
            Ok(s)=>{
                let   arc = Arc::new(Mutex::new(s));

                let data1 = Arc::clone(&arc);
                std::thread::spawn(move||{
                    let data_object = data1.lock().unwrap();
                utils::show_messages(format!("new connected device {:?}",data_object.peer_addr().unwrap()));
                });

                let   data2 = Arc::clone(&arc);

                std::thread::spawn(move||{
                let mut buffer = [0;1024];

                let mut  data_read_object = data2.lock().unwrap();
                
                let string_size = match  data_read_object.read(&mut buffer) {
                    Ok(n)=>n,
                    Err(e)=>{
                        utils::exit_program(format!("{}",e));
                    }
                    
                };

                drop(data_read_object);

                let  cmd = model::conevert_bytes_to_command(&buffer[..string_size]);
                println!("the gotten cmd is {:?}",cmd);
               match cmd {
        model::Command::Get { key } => {
        println!("Received GET for key: {}", key);

        //  get the key and return an response 

        let does_data_exist =  operation::does_data_exists_already(key.clone());
        
        let response: model::GetCommandResponse ;
        

        if does_data_exist{
            let fetched_data = operation::get_data(key.clone());
            match fetched_data {
                None =>{
                response =     model::GetCommandResponse{
                success: false,
                message: format!("can't find the key: {}   ",key.clone())
                 };
                }
                Some(s)=>{
                response =     model::GetCommandResponse{
                success: true,
                message: format!("value : {}   ",s)
                 };
                }
                
            }
        }else{
         response =     model::GetCommandResponse{
                success: false,
                message: format!("the key does not exists: {}   ",key)
        };
        }



        
            let mut  data_write_object  = data2.lock().unwrap();
            let success_response_bytes = model::convert_get_response_to_bytes(&response);

            data_write_object.write_all(&success_response_bytes).unwrap();  

        }
        model::Command::Set { key, value } => {
         println!("Received SET for key: {}, value: {}", key, value);

        //  save the key and return a response

           let does_data_exist =  operation::does_data_exists_already(key.clone());

            let response : SetCommandResponse;
            println!("does data exist : {}",does_data_exist);
           if does_data_exist {
            response = SetCommandResponse { success: false, message: "key already exists.please use another key".to_string() }
           }else{
             response = model::SetCommandResponse{
                success: true,
                message: format!("successfully saved key : {}   and value : {}",key,value)
            };
            operation::set_data(key.clone(), value.clone());
           }
            
            let mut  data_write_object  = data2.lock().unwrap();
            let response_bytes = model::convert_set_response_to_bytes(&response);

            data_write_object.write_all(&response_bytes).unwrap();
        }
        model::Command::Upload { filename, content }=>{
            

        }
        _ => {
        println!("Unknown command");
        }
    }



                });
            }
            Err(e)=>{
                utils::show_error_messages(e.to_string());
            }
            
        }
    }
}

