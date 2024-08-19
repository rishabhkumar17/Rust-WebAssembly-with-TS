#[derive(Debug)]
enum PersonId {
    // either Passport or IdentityCard
    Passport(String),
    IdentityCard(String),
}

struct Person {
    name: String, // fields
    age: u32,
    id: PersonId,
}

impl Person {
    //constructors
    fn new() -> Person {
        Person {
            name: "Default".to_string(),
            age: 0,
            id: PersonId::IdentityCard("ID123".to_string()),
        }
    }

    fn from(name: String, age: u32, id: PersonId) -> Person {
        Person { name, age, id }
    }

    // associated function -> can only be assesed by struct
    fn some_function() {
        println!("Yeah Buddy!")
    }

    //method
    // first parameter is always self which represent the instance of struct, the method is being called on
    // Within an impl block, the type Self is an alias for current type
    fn display_age(&self) {
        println!("Current age: {}", self.age);
    }

    fn change_age(&mut self, new_age: u32) {
        self.age = new_age;
    }
}

fn main() {
    Person::some_function();

    let mut person: Person = Person::new();
    let person_2: Person = Person::from(
        "sage".to_string(),
        17,
        PersonId::Passport("Pass123".to_string()),
    );

    person.display_age();

    person.change_age(28);

    println!("{} {} {:?}", person.name, person.age, person.id);
    println!("{} {} {:?}", person_2.name, person_2.age, person_2.id);
}
