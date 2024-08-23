use rust_webassembly_with_ts::logger;
use rust_webassembly_with_ts::logger_2;
use rust_webassembly_with_ts::Animal;
use rust_webassembly_with_ts::Person;

fn main() {
    let person = Person::new();
    let animal = Animal("dog".to_string(), 2, "indie".to_string());

    // person.alert_something();
    // animal.alert_something();

    // logger(animal);
    logger(person);

    logger_2(&animal);
}
