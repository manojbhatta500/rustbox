use crate::model;



pub fn  convert_command_to_bytes_test(){
    let test_data = model::Command::Set { key: "hello".to_string(), value: "world".to_string() };
    model::convert_command_to_bytes(&test_data);
}