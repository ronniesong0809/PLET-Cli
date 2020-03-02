# PLET-Cli
![Rust](https://github.com/ronniesong0809/plte-cli/workflows/Rust/badge.svg)

Copyright (c) 2020 Ronnie Song

This is a Rust based Command-line tool sets that provided scraping PLET (Portland Local Tech Events) data from [Calagator](https://calagator.org/), saving it as cvs file, import csv it to update MongoDB collection, display documents, and more.

-  [Calagator](https://calagator.org/) is an open source community calendar platform.
-  This project restructuring is inspired and by [portland-local-tech-event](https://github.com/ronniesong0809/portland-local-tech-event) and [plte-scrapper](https://github.com/ronniesong0809/plte-scrapper).

## Usage

#### Setup
Please install prerequisites MongoDB: [here](https://docs.mongodb.com/manual/administration/install-community/)

```shell
$ sudo apt-get update
$ sudo apt-get install -y pkg-config libssl-dev mongodb
$ cargo build
```

#### Run

To run the example program, type the command below:

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
test common::collection_connection_failed ... ok
test common::collection_connection_test ... ok
test common::database_connection_test ... ok
test common::scraping_html_test ... ok
test common::scraping_json_test ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

All tests passed with no issues.

The tests are placed in tests/test.rs file that uses std assert_eq!() and assert_ne!() to test equality of the actual result and expected result of the MongoDB connection and Scrapping status code in that file.

Github Action CI are running to do the automated testing.
![Rust](https://github.com/ronniesong0809/plte-cli/workflows/Rust/badge.svg)

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
