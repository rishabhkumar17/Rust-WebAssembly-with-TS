// use rust_webassembly_with_ts::logger;
// use rust_webassembly_with_ts::logger_2;
// use rust_webassembly_with_ts::Animal;
// use rust_webassembly_with_ts::Person;

// use rust_webassembly_with_ts::*;
use rust_webassembly_with_ts::learning_rust::{top_level::{self, low_level}, Log, Person};
// use std;

fn main() {
    let person = Person::new();

    // absolute path
    // crate::top_level::test_1();
    // crate::low_level::test_1();

    // relative path
    // top_level::test_1();
    // low_level::test_1();
    // person.display_info();

    println!("{}", person.name());
}
