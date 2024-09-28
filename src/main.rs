pub fn raindrops(input: u32) -> String {
    let mut result = String::new();

    // Check divisibility and append corresponding sounds
    if input % 3 == 0 {
        result.push_str("Pling");
    }
    if input % 5 == 0 {
        result.push_str("Plang");
    }
    if input % 7 == 0 {
        result.push_str("Plong");
    }

    // If no factors matched, return the number as a string
    if result.is_empty() {
        input.to_string()
    } else {
        result
    }
}

fn main() {
    // Test cases to demonstrate raindrops function
    let inputs = [28, 30, 34, 105]; // Example numbers to test
    for &input in &inputs {
        println!("Input: {}, Raindrop sound: {}", input, raindrops(input));
    }
}
