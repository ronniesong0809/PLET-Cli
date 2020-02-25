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
    // println!("{:?}", status);
    println!("{:?}", body);
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

fn month_choose() -> &'static str {
    loop{
    println!("Enter the number (ex:1 to Jan, 2 to Feb) of the month: ");
    print!("\n> ");

    let input = user_input();
    let command = input.trim().split_whitespace().next().unwrap();
    match &*command {
        "1" => return "Jan",
        "2" => return "Feb",        
        "3" => return "Mar",        
        "4" => return "Apr",        
        "5" => return "May",        
        "6" => return "Jun",   
        "7" => return "Jul",        
        "8" => return "Aug",        
        "9" => return "Sep",        
        "10" => return "Oct",        
        "11" => return "Nov",
        "12" => return "Dec",
        _ => println!("[{}]: command not found, please try again!",command),
    }
    }
}
/*
fn day_choose() -> () {
    println!("Enter the number (ex:1 to First day, 2 to Second day) of the day: ");
    print!("\n> ");

    let input = user_input();
    let command = input.trim().split_whitespace().next().unwrap();
    command;
}

fn year_choose() -> () {
    println!("Enter the number (ex:1995, 2000, 2020) of the year: ");
    print!("\n> ");

    let input = user_input();
    let command = input.trim().split_whitespace().next().unwrap();
    command;
}
*/

pub fn search(coll_name: &str, field: &str) {
    let coll = coll(coll_name);
    if(field == "1"){
        for doc in coll.find(None,None).unwrap() {
        println!("{}", doc.unwrap());
      }
    }
    else if(coll_name == "calendar" && field == "4") {
        let month = month_choose();

        println!("Enter the number (ex:1 to First day, 2 to Second day) of the day: ");
        print!("\n> ");
        let input = user_input();
        let temp = input.trim().split_whitespace().next().unwrap();

        println!("Enter the number (ex:1995, 2000, 2020) of the year: ");
        print!("\n> ");
        let input = user_input();
        let tempy = input.trim().split_whitespace().next().unwrap();
        let filter = doc!{"month":month,"day":temp,"year":tempy};
        let cursor = coll.find(filter,None).unwrap();
        for result in cursor {
            match result {
                Ok(document) => println!("\nDocument: {:?}", document),
                Err(e) => println!("Error! {:?}", e),
            }
        }
    }
    else {
        print!("\nPlease enter the {} plan to search: " ,field);
        let find_input = user_input();
        let find = find_input.trim();
        let filter = doc! { field: find };
        let cursor = coll.find(filter, None).unwrap();

        for result in cursor {
            match result {
                Ok(document) => println!("\nDocument: {:?}", document),
                Err(e) => println!("Error! {:?}", e),
            }
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
