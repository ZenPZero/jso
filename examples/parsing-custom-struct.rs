#![allow(unused)]

#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
  alive: bool,
}

const PEOPLE: [&str; 14] = [
  r#"{ "name": "Zen", "age": 10, "alive": true }"#,
  "",
  ";",
  "0",
  "{}",
  r#"{ "name": 0 }"#,
  r#"{ "name": "  " }"#,
  r#"{ "name": "Katya" }"#,
  r#"{ "name": "Katya", "age": null }"#,
  r#"{ "name": "Katya", "age": 0.1 }"#,
  r#"{ "name": "Katya", "age": -1 }"#,
  r#"{ "name": "Katya", "age": 8 }"#,
  r#"{ "name": "Katya", "age": 8, "alive": [] }"#,
  r#"{ "name": "Katya", "age": 8, "alive": false }"#,
];

fn main() {
  for json in PEOPLE {
    match parse_person(json) {
      Ok(p) => println!("{p:#?}"),
      Err(err) => eprintln!("failed to parse r#\"{json}\"#: {err:?}"),
    }
  }
}

#[derive(Debug)]
enum ParsePersonError {
  JsonParse(jso::parse::Error),
  JsonNotAnObject,
  Custom(&'static str),
}

fn parse_person(s: &str) -> Result<Person, ParsePersonError> {
  use ParsePersonError::*;

  let json = jso::parse(s).map_err(JsonParse)?;
  let mut o = json.obj().ok_or(JsonNotAnObject)?;

  let name = o
    .remove("name")
    .ok_or(Custom("missing \"name\""))?
    .str()
    .ok_or(Custom("\"name\" is not a string"))?
    .trim()
    .to_string();

  if name.is_empty() {
    return Err(Custom("\"name\" empty"));
  }

  let age = o
    .remove("age")
    .ok_or(Custom("missing \"age\""))?
    .num()
    .ok_or(Custom("\"age\" is not a number"))?;

  if age.fract() != 0. {
    return Err(Custom("\"age\" must be an integer"));
  }

  if !(0.0..=255.0).contains(&age) {
    return Err(Custom("\"age\" must be between 0-255"));
  }

  let age = age as u8;

  let alive = o
    .remove("alive")
    .ok_or(Custom("missing \"alive\""))?
    .bool()
    .ok_or(Custom("\"alive\" not a boolean"))?;

  Ok(Person { name, age, alive })
}
