use crate::person::Person;

mod country;
mod person;

fn main() {
    let person = Person::new("Diego", "Doná");
    let first_name = person.get_first_name();

    println!("The first name is: {}", first_name);

    let new_first_name = "Diogo";
    match person.set_first_name(new_first_name) {
        Ok(_) => {}
        Err(message) => {
            println!("sorry, shit happens: {}", message);
            return;
        }
    }

    println!("The NEW first name is: {}", person.get_first_name());
}
