use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::BinaryHeap;

fn main() {     
    let path = Path::new("data/input.dat");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => process_data(s),
    }
}

fn process_data(data : String) {
    let mut heap = BinaryHeap::new();
    let mut current_calories : u32 = 0;
    for line in data.lines() {
        match line.parse::<u32>() {
            Err(_) => {
                println!("{}", current_calories);
                heap.push(current_calories);
                current_calories = 0;
            },
            Ok(calories) => current_calories += calories,
        };
    }

    if(current_calories > 0)
    {
        println!("{}", current_calories);
        heap.push(current_calories);
    }

    let highest = heap.pop().unwrap_or(0);
    let mut highest_three = highest;
    highest_three += heap.pop().unwrap_or(0);
    highest_three += heap.pop().unwrap_or(0);

    println!("Highest Calories: {}\nHighest Three: {}", highest, highest_three);
}