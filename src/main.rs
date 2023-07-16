use std::collections::HashMap;
use rand::{distributions::Uniform, Rng};

fn create_integer_list() -> Vec<i32> {
    let range = Uniform::from(0..20);
    let integer_list = rand::thread_rng().sample_iter(&range).take(100).collect();
    integer_list
}

fn average_of_numbers(list: &[i32]) -> i32 {
    list.iter().sum::<i32>() / list.len() as i32
}

fn median_of_numbers(list: &mut [i32]) -> i32 {
    list.sort();
    let middle = list.len() / 2;
    if list.len() % 2 == 0 {
        (list[middle - 1] + list[middle]) / 2
    } else {
        list[middle]
    }
}

fn mode_of_numbers(list: &[i32]) -> i32 {
    struct Highest {
        key: i32,
        count: i32,
    }
    let mut mode = HashMap::new();
    let mut highest = Highest {key: 0, count: 0};
    for key in list {
        let count = mode.entry(key).or_insert(0);
        *count += 1;
        if *count > highest.count {
            highest.count = *count;
            highest.key = *key;
        }
    }
    highest.key
}
        
fn main() {
    let integer_list = create_integer_list();
    let average = average_of_numbers(&integer_list);
    let mut median_list = integer_list.clone();
    let median = median_of_numbers(&mut median_list);
    let mode = mode_of_numbers(&integer_list);
    println!("The list is: {:?}", integer_list);
    println!("The average is: {}", average);
    println!("The median is: {}", median);
    println!("The mode is: {}", mode);
}
