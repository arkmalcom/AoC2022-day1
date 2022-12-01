use std::collections::BinaryHeap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt").expect("File not found");
    let buf_reader = BufReader::new(file);
    let mut calories_sum = 0;
    let mut calories_list = Vec::new();

    for line in buf_reader.lines() {
        let value = line?;

        if value.is_empty() {
            calories_list.push(calories_sum);
            calories_sum = 0;
            continue;
        }

        calories_sum += value.parse::<i32>().unwrap();
    }

    let top_calorie_count = get_top_n_calorie_counts(&calories_list, 1);
    println!("The elf with the most calories is carrying {} calories.", top_calorie_count.iter().next().unwrap());

    let top_three_calorie_count = get_top_n_calorie_counts(&calories_list, 3);
    let mut top_three_sum: i32 = 0;

    for (index, calorie_count) in top_three_calorie_count.iter().enumerate() {
        println!("The top {} elf is carrying {} calories", index + 1, calorie_count);
        top_three_sum += calorie_count;
    }

    println!("The top three elves are carrying {} calories.", top_three_sum);
    Ok(())
}

fn get_top_n_calorie_counts(calories_list: &Vec<i32>, n: i32) -> Vec<i32> {
    let mut heap = calories_list.iter().copied().collect::<BinaryHeap<i32>>();
    let mut top_n_carriers = Vec::new();

    for _ in 0..n {
        if let Some(v) = heap.pop() {
            top_n_carriers.push(v);
        }
    }

    return top_n_carriers;
}