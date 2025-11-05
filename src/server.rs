use std::{io::Read, net::TcpListener, sync::{Arc, Mutex}};

use crate::{model, utils};

pub fn start_tcp_server(port_no : i32)->Result<TcpListener, std::io::Error>{
    let full_address = format!("127.0.0.1:{}", port_no);
    let  tcp_listener_object = TcpListener::bind(full_address)?;
    Ok(tcp_listener_object)
}


pub fn handle_connection(tcp_object : &TcpListener){
    for stream in tcp_object.incoming(){
        match stream {
            Ok(s)=>{
                let mut  arc = Arc::new(Mutex::new(s));

                let data1 = Arc::clone(&arc);
                std::thread::spawn(move||{
                    let data_object = data1.lock().unwrap();
                utils::show_messages(format!("new connected device {:?}",data_object.peer_addr()));
                });

                let data2 = Arc::clone(&arc);

                std::thread::spawn(move||{
                let mut buffer = [0;1024];

                let mut  data_object2 = data2.lock().unwrap();
                
                let string_size = match  data_object2.read(&mut buffer) {
                    Ok(n)=>n,
                    Err(e)=>{
                        utils::exit_program(format!("{}",e));
                    }
                    
                };

                let  cmd = model::conevert_bytes_to_command(&buffer[..string_size]);

               match cmd {
        model::Command::Get { key } => {
        println!("Received GET for key: {}", key);
        }
        model::Command::Set { key, value } => {
        println!("Received SET for key: {}, value: {}", key, value);
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

