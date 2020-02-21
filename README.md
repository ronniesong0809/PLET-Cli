# rust-project
[![Build Status](https://travis-ci.com/ronniesong0809/rust-project.svg?branch=master)](https://travis-ci.com/ronniesong0809/rust-project)

Copyright (c) 2020 Ronnie Song

This is a Rust based Command-line tool sets that scraping events from [Calagator](https://calagator.org/), saving it as cvs file, save it to MongoDB, and more. 

-  [Calagator](https://calagator.org/) is an open source community calendar platform.
-  This project restructuring is inspired and by [portland-local-tech-event](https://github.com/ronniesong0809/portland-local-tech-event) and [plte-scrapper](https://github.com/ronniesong0809/plte-scrapper).

## Usage

#### Setup
```shell
$ sudo apt-get install pkg-config libssl-dev
$ cargo build
```

#### Run
```shell
$ cargo run
```

## References
-  Rust Web Scraping by Gokberk Yaltirakli: [here](https://www.gkbrk.com/wiki/rust_web_scraping/)
-  Rust Docs - csv::Writer: [here](https://docs.rs/csv/1.0.0-beta.1/csv/struct.Writer.html)
-  Read Cookbook - csv records: [here](https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html#csv-processing)
-  Official MongoDB Rust Driver: [here](https://www.mongodb.com/blog/post/announcing-the-official-mongodb-rust-driver)
-  Dependencies: [here](https://github.com/ronniesong0809/rust-project/blob/master/Cargo.toml)

## License

This program is licensed under the "MIT License". Please see the file [`LICENSE`](https://github.com/ronniesong0809/rust-project/blob/master/LICENSE) in the source distribution of this software for license terms.
