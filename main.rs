#[derive(Debug)]
enum PersonId {
    // either Passport or IdentityCard
    Passport(String),
    IdentityCard(String, u32, u32),
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
            id: PersonId::IdentityCard("ID".to_string(), 1, 2),
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

    fn display_info(&self) {
        println!("{} {} {:?}", self.name, self.age, self.id);
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

    person.display_info();

    person_2.display_info();

    check_person_id(person.id);

    check_person_id(person_2.id);
}

fn check_person_id(id: PersonId) {
    if let PersonId::Passport(ref num) = id {
        println!("Matching");
    } else {
        println!("Not Matching");
    }
    
    match id {
        PersonId::IdentityCard(x, y, z) => {
            println!("{} {} {}", x, y, z);
        }
        PersonId::Passport(x) => {
            println!("{}", x)
        }
    }
    
}
