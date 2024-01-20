use crate::person::Person;

mod country;
mod person;

fn main() {
    println!("\n");
    println!("*******************************");
    println!("******** Changing Names *******");
    println!("*******************************");
    println!("\n");

    let person = Person::new("Diego", "Doná", 1989);
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

    println!("\n");
    println!("*******************************");
    println!("********* Greetings ***********");
    println!("*******************************");
    println!("\n");

    let other_person = Person::new("José", "Oliveira", 1975);
    match person.greet_other_person(&other_person) {
        Some(greeting) => println!(
            "{} greeted {} with '{}'",
            person.get_first_name(),
            other_person.get_first_name(),
            greeting
        ),
        None => println!("There seems nothing was to be greeted between those two."),
    }
}
