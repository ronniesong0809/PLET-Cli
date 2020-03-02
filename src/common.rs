extern crate mongodb;
use bson::{bson, doc};
use mongodb::{options::ClientOptions, Client};
use std::{
    fs::File,
    io::{stdin, stdout, BufReader, Write},
};

#[tokio::main]
pub async fn scraping(url: &str, dist: &str) -> Result<reqwest::StatusCode, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let status = response.status();
    let body = response.text().await?;
    println!("status: {:?}", status);
    // println!("{:?}", body);
    let mut file = File::create(dist)?;
    file.write_all(body.as_bytes())?;
    file.sync_all()?;
    Ok(status)
}

fn mongodb_driver() -> mongodb::Client {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
    client_options.app_name = Some("My app".to_string());
    Client::with_options(client_options).unwrap()
}

fn db() -> mongodb::Database {
    let client = mongodb_driver();
    client.database("calagator")
}

pub fn coll(coll_name: &str) -> mongodb::Collection {
    db().collection(coll_name)
}

pub fn read_file(file: File) -> csv::Reader<std::io::BufReader<std::fs::File>> {
    csv::Reader::from_reader(BufReader::new(file.try_clone().unwrap()))
}

pub fn display(coll_name: &str) {
    let coll = coll(coll_name);
    let cursor = coll.find(None, None).unwrap();
    for doc in cursor {
        println!("\n{}", doc.unwrap());
    }
}

pub fn search(coll_name: &str, field: &str) {
    print!("\nPlease enter: ");
    let find_input = user_input();
    let find = find_input.trim();
    // print!("{}", find);
    let coll = coll(coll_name);
    let filter = doc! { field: find };
    // let find_options = FindOptions::builder()
    //     .sort(doc! { "State": 1 })
    //     .build();
    let cursor = coll.find(filter, None).unwrap();

    for result in cursor {
        match result {
            Ok(document) => println!("\nDocument: {:?}", document),
            Err(e) => println!("Error! {:?}", e),
        }
    }
}

pub fn user_input() -> std::string::String {
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    // print!("{:?}", input);
    if input == "\n" {
        input = "0".to_string();
    }
    input
}

#[test]
fn database_connection_test() {
    let client = mongodb_driver();
    let coll = client.database("calagator").collection("venues");
    coll.insert_one(doc! { "id": "1"}, None).unwrap();

    let db_name = client.list_database_names(None).unwrap();
    let check = (&db_name).into_iter().any(|v| v == "calagator");

    assert_eq!(check, true);
}

#[test]
fn database_connection_failed() {
    let client = mongodb_driver();
    let db_name = client.list_database_names(None).unwrap();
    let check = (&db_name).into_iter().any(|v| v == "not_exist");

    assert_ne!(check, true);
}

#[test]
fn collection_connection_test() {
    let db = db();
    let coll = db.collection("venues");
    coll.insert_one(doc! { "id": "1"}, None).unwrap();

    let coll_name = db.list_collection_names(None).unwrap();
    let check = (&coll_name).into_iter().any(|v| v == "venues");

    assert_eq!(check, true);
}

#[test]
fn collection_connection_failed() {
    let client = db();
    let coll_name = client.list_collection_names(None).unwrap();
    let check = (&coll_name).into_iter().any(|v| v == "not_exist");

    assert_ne!(check, true);
}

#[test]
fn scraping_html_test() {
    let html = scraping("https://calagator.org", "assets/calendar.html").unwrap();

    assert_eq!(html.as_u16(), 200);
}

#[test]
fn scraping_json_test() {
    let json = scraping("https://calagator.org/events.json", "assets/events.json").unwrap();

    assert_eq!(json.as_u16(), 200);
}
