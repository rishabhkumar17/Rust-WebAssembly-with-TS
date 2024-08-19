struct Person {
    name: String, // fields
    age: u32,
}

impl Person {
    // associated function -> can only be assesed by struct
    fn some_function() {
        println!("Yeah Buddy!")
    }

    //method
    // first parameter is always self which represent the instance of struct, the method is being called on
    // Within an impl block, the type Self is an alias for current type
    fn change_age(&self) {
        println!("Current age: {}", self.age);
    }
}

fn main() {
    Person::some_function();

    let person: Person = Person {
        name: "sage".to_string(),
        age: 27,
    };

    person.change_age();

    println!("{} {}", person.name, person.age);
}
