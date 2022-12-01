mod another_lib;

use another_lib::another_mod::another_fn;

fn outside_function() {
    println!("Written from outside function");
    crate::another_lib::another_mod::another_fn();
    another_fn();
}

pub mod education {

    pub mod learning_structs {
        use std::fmt::{Debug, Display, Formatter, Result as FormatResult};

        mod top_level {
            pub fn hi_there() {
                println!("Hi there!");
            }

            pub mod low_level {
                pub fn hello_world() {
                    super::hi_there();
                    println!("Hello world!");
                }
            }
        }

        pub trait Log {
            fn display_info(&self);
        }

        #[derive(Debug)]
        pub enum PersonId {
            Passport(u32),
            IdCard(u32, u32),
        }

        impl Display for PersonId {
            fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
                match self {
                    PersonId::Passport(a) => write!(f, "Passport ({})", a),
                    PersonId::IdCard(a, b) => write!(f, "IdCard ({}, {})", a, b),
                }
            }
        }

        #[derive(Debug)]
        pub struct Person {
            pub first_name: String,
            pub last_name: String,
            age: u32,
            id: PersonId,
        }

        impl Person {
            pub fn new(first_name: &str, last_name: &str, age: u32, id: PersonId) -> Self {
                Self {
                    first_name: first_name.to_string(),
                    last_name: last_name.to_string(),
                    age,
                    id,
                }
            }

            pub fn introduce(&self) {
                println!(
                    "Hello, my name is {} {} and I am {} years old.\r\nMy Id is: {}",
                    self.first_name, self.last_name, self.age, self.id
                );
            }

            pub fn change_id(&mut self, new_id: PersonId) {
                self.id = new_id;
            }
        }

        impl Display for Person {
            fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
                write!(
                    f,
                    "{} {}, {} ({})",
                    self.first_name, self.last_name, self.age, self.id
                )
            }
        }

        impl Log for Person {
            fn display_info(&self) {
                println!("Log :: {} {}", self.first_name, self.last_name);

                // relative path
                top_level::hi_there();
                top_level::low_level::hello_world();

                // absolute path
                // crate::learning_structs::top_level::hi_there();
                // crate::learning_structs::top_level::low_level::hello_world();

                crate::education::learning_structs::top_level::hi_there();
                crate::education::learning_structs::top_level::low_level::hello_world();

                crate::outside_function();
                // super::outside_function();
                super::super::outside_function();
            }
        }

        #[derive(Debug)]
        pub struct Animal(pub String, pub u32, pub String);

        impl Log for Animal {
            fn display_info(&self) {
                println!("Log :: {} {} {}", self.0, self.1, self.2)
            }
        }

        // Move
        pub fn log_info_move(val: impl Log) {
            val.display_info();
        }

        // Borrow with dynamic dispatch
        pub fn log_info_borrow(val: &dyn Log) {
            val.display_info();
        }

        // Borrow with static dispatch
        pub fn log_info_borrow_static(val: &impl Log) {
            val.display_info();
        }
    }
}
