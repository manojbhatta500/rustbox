use crate::model::{DataVault, DATA_VAULT};

pub fn set_data(key : String, value : String){
    let mut vault = DATA_VAULT.lock().unwrap();
    vault.push(DataVault { key, value });
}


pub fn get_data(key : String, value : String)-> Option<String>{
    let  vault = DATA_VAULT.lock().unwrap();
    for entry in vault.iter(){
        if entry.key == key{
            return  Some(entry.value.clone());
        }
    }
    None
}


pub fn update_data(key : String, new_value : String)-> bool{
    let  mut  vault = DATA_VAULT.lock().unwrap();
    for entry in vault.iter_mut(){
        if entry.key == key{
            entry.value = new_value;
            return  true;
        }
    }

    return  false;
}

