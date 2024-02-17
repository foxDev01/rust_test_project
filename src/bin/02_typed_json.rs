use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

// парсим json зная структуру
#[derive(Deserialize, Serialize, Debug)]
struct Person {
    name: String,
    gender: String,
    age:i32,

    #[serde(default)]
    cars:Vec<Car>
}
#[derive(Deserialize, Serialize, Debug)]
struct  Car {
    id: i32,
    name: String
}
fn main() {
    let mut people = {
        let res = fs::read_to_string("data.json").expect("Не удалось открыть файл");
        serde_json::from_str::<Vec<Person>>(&res).unwrap()
    };

    people[1].name = "Lolik".to_string();
    println!("{:?}",people);
}