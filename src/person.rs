use std::cell::Cell;

use crate::country::Country;

#[allow(dead_code)]
pub struct Person<'a> {
    first_name: Cell<&'a str>,
    last_name: Cell<&'a str>,
    pub birth_year: u16,
    pub country_origin: Country,
}

impl<'a> Person<'a> {
    pub fn new(first_name: &'static str, last_name: &'static str, birth_year: u16) -> Person<'a> {
        return Person {
            first_name: Cell::from(first_name),
            last_name: Cell::from(last_name),
            birth_year,
            country_origin: Country::Brazil,
        };
    }

    pub fn get_first_name(&self) -> &str {
        return self.first_name.get();
    }

    pub fn set_first_name(&self, new_first_name: &'a str) -> Result<(), &str> {
        if new_first_name.len() < self.first_name.get().len() {
            return Err("new name cant be smaller than the original");
        }

        self.first_name.set(new_first_name);
        return Ok(());
    }

    pub fn greet_other_person(&self, other_person: &Person) -> Option<String> {
        if other_person.birth_year == self.birth_year {
            return Some("We were born in the same year!".to_string());
        }

        if self.birth_year < other_person.birth_year {
            return Some(format!(
                "Haha, I'm older than you, {}!",
                other_person.get_first_name()
            ));
        }

        return None;
    }
}
