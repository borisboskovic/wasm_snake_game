use snake_game::education::learning_structs::{Log, Person, PersonId};

fn main() {
    let person = Person::new("John", "White", 26, PersonId::IdCard(32, 64));
    // person.display_info();
    // println!("{} {}", person.first_name, person.last_name);
    println!("{}", person);
}
