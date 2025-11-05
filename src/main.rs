use std::env;


mod utils;
mod engine;
mod server;
mod model;
mod test;




fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        utils::exit_program("please provide the port for rustbox server".to_string());
    }

    let port = match utils::parse_number(&args[1]) {
        Ok(n) => n,
        Err(e) => {
            utils::exit_program(e);
        }
    };

    let tcp_listener = match server::start_tcp_server(port) {
        Ok(listener) => listener,
        Err(e) => {
            utils::exit_program(e.to_string());
        }
    };


    utils::show_messages(format!("rustbox is running on port {}",port));

    server::handle_connection(&tcp_listener);



}


// for testing
// fn main(){
//     test::convert_command_to_bytes_test();
// }