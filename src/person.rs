use std::cell::Cell;

use crate::country::Country;

#[allow(dead_code)]
pub struct Person<'a> {
    first_name: Cell<&'a str>,
    last_name: Cell<&'a str>,
    birth_year: u16,
    country_origin: Country,
}

impl<'a> Person<'a> {
    pub fn new(first_name: &'static str, last_name: &'static str) -> Person<'a> {
        return Person {
            first_name: Cell::from(first_name),
            last_name: Cell::from(last_name),
            birth_year: 1989,
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
}
