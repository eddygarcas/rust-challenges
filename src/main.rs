extern crate num;

use std::fmt::{Display, Formatter, write};
use std::iter::Sum;
use std::ops::Add;
use chrono::{Date, Local, TimeZone};
use crate::Pulse::{Long, Short};
use crate::Scale::{Celsius, Fahrenheit};


/////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

type Letter = Vec<Pulse>;
type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

//So taking into account that the vector will contain string, we need to implement MorseCode as
//part of the String object
impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        //&self is the current String, the one that contains the message, so we need to interate that char by char
        //and map every char with it's morse code.
        use Pulse::*; //simplify the usage of the enum
        let mut msg = Vec::with_capacity(self.len());

        for c in self.chars() {
            let morse_code = match c {
                'A' | 'a' => vec![Short, Long],
                'B' | 'b' => vec![Short, Long, Short, Long],
                _ => continue,
            };
            msg.push(morse_code);
        }
        msg
    }
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

/////////////////////////////////////////////////

struct ImportantEvent {
    name: String,
    birthday: Date<Local>,
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.birthday < Local::today()
    }
}

/////////////////////////////////////////////////

enum Scale {
    Celsius,
    Fahrenheit,
}

struct Temperature {
    degrees: f32,
    scale: Scale,
}

trait ConvertTemperature {
    fn to_celsius(&self) -> f32;
    fn to_fahrenheit(&self) -> f32;
}

impl ConvertTemperature for Temperature {
    fn to_celsius(&self) -> f32 {
        match self.scale {
            Fahrenheit => (self.degrees - 32 as f32) * (5 / 9) as f32,
            _ => self.degrees
        }
    }

    fn to_fahrenheit(&self) -> f32 {
        match self.scale {
            Celsius => (self.degrees * (9 / 5) as f32) + 32 as f32,
            _ => self.degrees
        }
    }
}

fn main() {
    let list = vec![1.5, 3.0, 5.0, 8.8];
    assert_eq!(median(list), Some(5.5));

    let list_unique = vec![1, 6, 2, 5];
    assert_eq!(unique(list_unique), [1, 2, 5, 6]);

    let mut _first = String::from("Hola");
    //Method is expecting a reference, which means that inside the method may be modified.
    info(&_first);
    println!("Value post method {}", _first);
    info_to_string(&"Hola non string");
    info(&23i32);
    info_with_ref(&"Using as ref");

    let mut users = vec!["Tood", "amy"];//Mutable as it need to be sorted
    sort_username(&mut users);//User &mut as will pass a reference toward the mut vector
    assert_eq!(users, vec!["amy", "Tood"]);

    //CHALLENGE 5
    let actual_code = "ab".to_string().to_morse_code();
    let expected = vec![vec![Short, Long], vec![Short, Long, Short, Long]];
    assert_eq!(actual_code, expected);

    //CHALLENGE 6 check dates
    let xmas = ImportantEvent {
        name: String::from("Eduard"),
        birthday: Local.ymd(2014, 7, 8),
    };
    println!("Has {} passed its date {}", xmas.name, xmas.is_passed());

    //CHALLENGE 8 convert celsius and fahrenheit
    let current_temperature = Temperature {
        degrees: 0f32,
        scale: Celsius,
    };
    println!("Temperature in celsius {}", current_temperature.to_celsius());
    println!("Temperature in  fahrenheit {}", current_temperature.to_fahrenheit());

    //CHALLENGE 9
    let mut cadena = vec![Some(2), None, Some(3)];
    let result_missing_data = sum_missing_data(&mut cadena);
    println!("Result of missing data {:?}", result_missing_data);
}

//CHALLENGE 1
fn median(mut list: Vec<f64>) -> Option<f64> {
    println!("Vector is {:?}", (list.len() % 2));
    if list.is_empty() {
        return None;
    }
    let middle = list.len() / 2;
    // As f64 doesn't implement Ord trait we need to use sort_by to specify how to compare both numbers.
    list.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let med = if list.len() % 2 == 1 {
        list[list.len() / 2]
    } else {
        list[middle - 1] + list[middle] / 2.0
    };
    Some(med)
}

//CHALLENGE 2
//fn unique<T: std::cmp::Ord>(mut list: Vec<T>) -> Vec<T> {
//This can be simplified as Ord is part of the standard library
//No need to use Box<T> as we are using primitive objects other wise the compiler will complain about it.
fn unique<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    list.sort();
    list.dedup();
    list
}

//CHALLENGE 3
// T: Display + Debug means that this method will accept any type that implements both traits.
fn info<T: Display>(text: &T) {
    println!("Text is {}", text);
}

//another interesting way, ToString trait will take any type that can convert itself into String.
//but this way allocates memory so can actually be costly, so there is another way....
fn info_to_string<T: ToString>(text: &T) {
    println!("Test is {}", text.to_string());
}

//now will use AsRef which means, can your type T behave like a String slice?
fn info_with_ref<T: AsRef<str>>(text: &T) {
    println!("Test is {}", text.as_ref());
}

//CHALLENGE 4
//Just sorting won't work as the uppercase letters always comes first at sorting.
fn sort_username<T: AsRef<str> + Ord>(users: &mut Vec<T>) {
    //let's iterate through all elements of our list and turn them to lowercase
    //and then sort there.
    // This will take an element iterating and apply a method temporarily just for the sort operation
    //which means it won't alter the value in the final sort.
    users.sort_by_cached_key(|x| x.as_ref().to_lowercase());
}

//CHALLENGE 9
//fn sum_missing_data<T: num::PrimInt + Sum>(data: &mut Vec<Option<T>>) -> T {
fn sum_missing_data(data: &mut Vec<Option<i32>>) -> i32 {
    //Functional programming, will iterate through Vec mapping every element, with either the
    //integer value or 0 and lastly will sum the whole array of i32
    data.iter().map(|x| x.unwrap_or(0)).sum()
}


#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![5.0, 3.0, 1.5];
    let expected_output = Some(3.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_list() {
    let input = vec![8.8, 5.0, 3.0, 1.5];
    let expected_output = Some(5.5);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unique_even_list() {
    let input = vec![1, 5, 2, 7, 9, 9];
    let expected_output = vec![1, 2, 5, 7, 9];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}
