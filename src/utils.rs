use colored::Colorize;


pub fn parse_number(arg: &str) -> Result<i32, String> {
    match arg.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("'{}' is not a valid number!", arg)),
    }
}


pub fn exit_program(message: String)-> !{
    show_error_messages("exiting program  reason: ".to_string());
    show_error_messages(format!("{}",message));
    std::process::exit(1);
}

pub fn show_error_messages(message : String){
    println!("{}",message.red());
}

pub fn show_messages(message : String){
    println!("{}",message.blue());
}





