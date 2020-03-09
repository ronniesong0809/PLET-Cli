# PLET-Cli
[![Rust](https://github.com/ronniesong0809/plte-cli/workflows/Rust/badge.svg)](https://github.com/ronniesong0809/plte-cli/actions)

Copyright (c) 2020 Ronnie Song, Boxuan Zhang, Mingjue Wang

This is a Rust based Command-line tool sets that provided scraping PLET (Portland Local Tech Events) data from [Calagator](https://calagator.org/), saving it as cvs file, import csv it to update MongoDB collection, display documents, and more.

-  [Calagator](https://calagator.org/) is an open source community calendar platform.
-  This project restructuring is inspired and by [portland-local-tech-event](https://github.com/ronniesong0809/portland-local-tech-event) and [plte-scrapper](https://github.com/ronniesong0809/plte-scrapper).

### Project Participants
- Ronnie Song: rongsong@pdx.edu
- Boxuan Zhang: boxuan@pdx.edu
- Mingjue Wang: mingwang@pdx.edu

### What was built?
We attempted to implement a command line web scraper that collects data from calagater.org. The typical operations would include collecting events from the website and store them locally as either json or html files, converting json or html to csv files, importing csv to local mongodb collection as documents, and then displaying document by queries.

### What it does?
1. Scrape and save [events](https://calagator.org/events)/[venues](https://calagator.org/venues) data to either **html/csv** files by [reqwest](https://crates.io/crates/reqwest) crate.
2. Parse and read html tags from the **html** file by [Select](https://crates.io/crates/select) library crate 
3. Parse and read objects from the **json** file by [JSON](https://crates.io/crates/serde_json) library crate
4. Stored parsed data to **csv** file by [CSV](https://crates.io/crates/csv) library crate
5. Read **csv** files and import data to MongoDB by the [MongoDB](https://crates.io/crates/mongodb) library crate
6. Display document from the MongoDB collection by document’s field.

## Usage

#### Setup
Please install prerequisites MongoDB: [here](https://docs.mongodb.com/manual/administration/install-community/) or run this script (Linux user):

```shell
$ sudo apt-get update
$ sudo apt-get install -y pkg-config libssl-dev mongodb
$ cargo build
```

#### Run

To run the program, type the command below:

```shell
$ cargo run
```

Everything went well! It's able to scrape events and save it to the right path, prase and convert to csv format from json/html without issues, and successfully import to MongoDB as expected.

#### Test

To test the library crate, type the command below:

```shell
$ cargo test
```
```shell
   Compiling plte_cli v0.2.0 (/mnt/d/Ronnie/Desktop/Winter 2020/rust/plte-cli)
    Finished test [unoptimized + debuginfo] target(s) in 34.29s
     Running target/debug/deps/plte_cli-12cc08ee9875ab00

running 6 tests
test common::database_connection_failed ... ok
test common::database_connection_test ... ok
test common::collection_connection_failed ... ok
test common::collection_connection_test ... ok
test common::scraping_html_test ... ok
test common::scraping_json_test ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

All tests passed with no issues.

The tests are placed in tests/test.rs file that uses std assert_eq!() and assert_ne!() to test equality of the actual result and expected result of the MongoDB connection and Scrapping status code in that file.

Github Action CI is running to do the automated testing.
[![Rust](https://github.com/ronniesong0809/plte-cli/workflows/Rust/badge.svg)](https://github.com/ronniesong0809/plte-cli/actions)

### Future Goal
- Scraping other websites.
- Have a process bar when importing a large data set.
- ensure that files exist or create files on the right path before starts.

### Lessons Learned
During the project I become more familiar with how to structure the program, use modules and bring them into files scope when necessary. I experimented with more standard rust methods and understood it by digging to the rust std source code. I believe these lessons I learned were also the main reason why people start liking Rust language and contributing a lot. Even though dynamically typed languages provide great flexibility, they often contain potential problems that are difficult to debug and can cause performance losses during execution. In statically typed languages, it catches and throws errors early in the compile process before run-time, and static typing usually results in faster execution of compiled code because it can generate optimized machine code when the compiler knows the exact data type being used.

## References
-  Rust Web Scraping by Gokberk Yaltirakli: [here](https://www.gkbrk.com/wiki/rust_web_scraping/)
-  Select.rs Library Crate Example: [here](https://github.com/utkarshkukreti/select.rs)
-  Rust Docs - csv::Writer: [here](https://docs.rs/csv/1.0.0-beta.1/csv/struct.Writer.html)
-  Read Cookbook - csv records: [here](https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html#csv-processing)
-  Official MongoDB Rust Driver: [here](https://www.mongodb.com/blog/post/announcing-the-official-mongodb-rust-driver)
-  Ascii Art by @[patorjk](https://github.com/patorjk?tab=repositories): [here](http://patorjk.com/)
-  Dependencies: [here](https://github.com/ronniesong0809/plte-cli/blob/master/Cargo.toml)

## License

This program is licensed under the "MIT License". Please see the file [`LICENSE`](https://github.com/ronniesong0809/plte-cli/blob/master/LICENSE) in the source distribution of this software for license terms.
