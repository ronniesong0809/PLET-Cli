# PLTE Project Writeup
Copyright (c) 2020 Ronnie Song, Mingjue Wang, Boxun Zhang

### What was built?
I attempted to implement a command line web scraper that collects data from calagater.org. The typical operations would include collecting events from the website and store them locally as either json or html files, converting json or html to csv files, importing csv to local mongodb collection as documents, and then displaying document by queries.

### What it does?
1. Scrape and save [events](https://calagator.org/events)/[venues](https://calagator.org/venues) data to either **html/csv** files by [reqwest](https://crates.io/crates/reqwest) crate.
2. Parse and read html tags from the **html** file by [Select](https://crates.io/crates/select) library crate 
3. Parse and read objects from the **json** file by [JSON](https://crates.io/crates/serde_json) library crate
4. Stored parsed data to **csv** file by [CSV](https://crates.io/crates/csv) library crate
5. Read **csv** files and import data to MongoDB by the [MongoDB](https://crates.io/crates/mongodb) library crate
6. Display document from the MongoDB collection by document’s field.
7. Delete document from the MongoDB collection.

### Future Goal?
- Scraping other websites.
- Have a process bar when importing a large data set.
- ensure that files exist or create files on the right path before starts.

### Lessons Learned
During the project I become more familiar with how to structure the program, use modules and bring them into files scope when necessary. I experimented with more standard rust methods and understood it by digging to the rust std source code. I believe these lessons I learned were also the main reason why people start liking Rust language and contributing a lot. Even though dynamically typed languages provide great flexibility, they often contain potential problems that are difficult to debug and can cause performance losses during execution. In statically typed languages, it catches and throws errors early in the compile process before run-time, and static typing usually results in faster execution of compiled code because it can generate optimized machine code when the compiler knows the exact data type being used.
