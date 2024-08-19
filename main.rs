struct Person {
    name: String, // fields
    age: u32,
}

impl Person {
    //constructors
    fn new() -> Person {
        Person {
            name: "Default".to_string(),
            age: 0,
        }
    }

    fn from(name: String, age: u32) -> Person {
        Person { name, age }
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
    let person_2: Person = Person::from("sage".to_string(), 17);

    person.display_age();

    person.change_age(28);

    println!("{} {}", person.name, person.age);
    println!("{} {}", person_2.name, person_2.age);
}
