mod another_lib;

fn outsider() {
    another_lib::another_mod::another_fn();
    println!("Outsider!")
}

pub mod education {

    pub mod learning_rust {

        pub mod top_level {
            pub fn test_1() {
                println!("Test 1")
            }

            pub mod low_level {
                pub fn test_1() {
                    println!("Test 1")
                }
            }
        }
        // trait
        pub trait Log {
            fn display_info(&self);
            fn alert_something(&self) {
                println!("Default")
            }
        }

        #[derive(Debug)]
        enum PersonId {
            // either Passport or IdentityCard
            Passport(String),
            IdentityCard(String, u32, u32),
        }

        pub struct Person {
            pub name: String, // fields
            age: u32,
            id: PersonId,
        }

        impl Log for Person {
            fn display_info(&self) {
                crate::outsider();

                // going outside of current module ( .. )
                super::super::outsider();

                super::super::another_lib::another_mod::another_fn();

                crate::another_lib::another_mod::another_fn();

                println!("{} {} {:?}", self.name, self.age, self.id);
            }
        }

        pub struct Animal(pub String, pub u32, pub String); // no fields

        impl Log for Animal {
            fn display_info(&self) {
                println!("{}", self.0);
            }
            fn alert_something(&self) {
                println!("Default Animal");
            }
        }

        impl Person {
            //constructors
            pub fn new() -> Person {
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

            pub fn name(&self) -> &String {
                &self.name
            }

            // fn display_info(&self) {
            //     println!("{} {} {:?}", self.name, self.age, self.id);
            // }
        }

        // impl makes the compiler determine type at the compile time
        // it will create mutiple versions of the funtion depends how many types Log traits implements(Animal, Person)
        pub fn logger(val: impl Log) {
            val.alert_something();
        }

        // dyn is short for dynamic, function performs dynamic dispatch -> dicision of exactly which function to call at the run time
        pub fn logger_2(val: &dyn Log) {
            val.alert_something();
        }
    }
}
