fn main() {
    let list = vec![1.5, 3.0, 5.0, 8.8];
    assert_eq!(median(list), Some(5.5));

    let list_unique = vec![1, 6, 2, 5];
    assert_eq!(unique(list_unique), [1, 2, 5, 6])
}

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

//fn unique<T: std::cmp::Ord>(mut list: Vec<T>) -> Vec<T> {
//This can be simplified as Ord is part of the standard library
//No need to use Box<T> as we are using primitive objects other wise the compiler will complain about it.
fn unique<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    list.sort();
    list.dedup();
    list
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
