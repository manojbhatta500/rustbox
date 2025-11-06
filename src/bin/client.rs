use std::{ env, net:: TcpStream};

use rustbox::utils;


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
    
    parse_command(&args);

    
}


fn connect_to_server(ip_address: String,port: String)->Result<TcpStream, std::io::Error>{
    let  server_full_address =  format!("{}:{}",ip_address,port);
    let tcp_stream= TcpStream::connect(server_full_address)?;
    Ok(tcp_stream)
}


fn parse_command(commands: &Vec<String>){
    let zero_index_upper_case = commands[0].to_ascii_uppercase();


    if commands[0].to_ascii_lowercase() == "help" {
        help_printer();
    }else if  zero_index_upper_case == "SET" ||
              zero_index_upper_case == "GET" ||
              zero_index_upper_case == "UPLOAD" ||
              zero_index_upper_case == "DOWNLOAD"
    {

        
    }else {
        utils::show_error_messages("invalid command. command must start with SET,GET,UPLOAD, DOWNLOAD".to_string());
        help_printer();
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
