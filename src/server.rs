use std::net::TcpListener;

use crate::utils;

pub fn start_tcp_server(port_no : i32)->Result<TcpListener, std::io::Error>{
    let full_address = format!("127.0.0.1:{}", port_no);
    let  tcp_listener_object = TcpListener::bind(full_address)?;
    Ok(tcp_listener_object)
}


pub fn handle_connection(tcp_object : &TcpListener){
    for stream in tcp_object.incoming(){
        match stream {
            Ok(s)=>{
                utils::show_messages(format!("new connection {:?}",s));
            }
            Err(e)=>{
                utils::show_error_messages(e.to_string());
            }
            
        }
    }
}

