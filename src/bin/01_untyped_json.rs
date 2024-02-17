
use std::fs;
use serde_json;
// парсим json не зная структуры 
fn main() {
    
    let res = fs::read_to_string("data.json");

    let s = match res {
        Ok(s)=> s,
        Err(_)=> panic!("Ошибка открытия файлва")
        
    };

    let mut json_data:serde_json::Value = serde_json::from_str(&s).expect("Не удалось спарсить json");
    println!("{json_data}");

    // изменение данных и сохранение в другой файл 
    json_data[0]["age"] =  serde_json::json!(123);
    json_data[0]["name"] = serde_json::json!("mlsko");

    println!("{json_data}");

    std::fs::write("output.json", serde_json::to_string_pretty(&json_data).unwrap()).expect("Не удалось сохранить файл");

}