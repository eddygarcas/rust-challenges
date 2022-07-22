use std::fmt::{Display, Formatter, write};
use crate::Pulse::{Long, Short};

//CHALLENGE 5
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

//CHALLENGE 6
enum Card {
    Ace,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand {
            cards: vec![],
        }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }
    fn value(&self) -> usize {
        let mut result: usize = 0;
        let mut aces_seen = 0;
        for c in &self.cards {
            use Card::*;
            result += match c {
                King | Queen | Jack => 10,
                //Here we can define an operation inside the match, just need to add it into brackets.
                //Remember to return a value anyway.
                Ace => {
                    aces_seen += 1;
                    0
                }
                One => 1,
                Two => 2,
                Three => 3,
                Four => 4,
                Five => 5,
                Six => 6,
                Seven => 7,
                Eight => 8,
                Nine => 9,
            }
        }

        for _ in 0..aces_seen {
            let ace_value = if result <= 10 { 11 } else { 1 };
            result += ace_value;
        }
        result
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

    let actual_code = "ab".to_string().to_morse_code();
    let expected = vec![vec![Short, Long], vec![Short, Long, Short, Long]];
    assert_eq!(actual_code, expected);

    let mut hand_cards = Hand::new();
    hand_cards.add(Card::King);
    hand_cards.add(Card::Ace);
    println!("Hand is {}", hand_cards.value())
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
