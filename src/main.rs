extern crate serde_json;
extern crate reqwest;

use std::io::Read;
use serde_json::{Value, Error};
use reqwest::Url;


#[allow(unused_must_use)]
fn get_temp(zip: &str) -> Result<(), Error> {
    let key = "2dd6e34dc478e049";
    let url = format!("http://api.wunderground.com/api/{}/conditions/q/{}.json", key, zip);
    
    let mut resp = reqwest::get(Url::parse(&url).unwrap()).unwrap();
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);

    let v: Value = serde_json::from_str(content.as_str())?;
    let ref current = v["current_observation"]["temperature_string"];
    println!("{}", current);

    Ok(())

}

fn main() {
    get_temp("97068");
}