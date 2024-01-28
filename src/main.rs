use std::io;
use std::io::Write;

// A program that calculates your heart rate from 55% to 95%.
// Input: Age, Resting heart rate
// Process: loop from 55 - 95, increment by 5. Use carvonen formula to calculate heart rate.
// Output: The intensity percentage and heart rate

fn calculate_heart_rate(age: i32, resting_heart_rate: i32, intensity_percentage: i32) -> i32 {
    // (((220 − age) − restingHR) × intensity) + restingHR
    let target_heart_rate: f32 = (((220.0 - age as f32) - resting_heart_rate as f32)
        * (intensity_percentage as f32 / 100.0))
        + resting_heart_rate as f32;

    target_heart_rate.round() as i32
}

#[cfg(test)]
mod tests {
    use super::calculate_heart_rate;

    #[test]
    fn test_calculate_heart_rate() {
        // age, resting_heart_rate, intensity_percentage
        assert_eq!(calculate_heart_rate(22, 65, 55), 138);
        assert_eq!(calculate_heart_rate(22, 65, 60), 145);
        assert_eq!(calculate_heart_rate(22, 65, 65), 151);
        assert_eq!(calculate_heart_rate(22, 65, 85), 178);
        assert_eq!(calculate_heart_rate(22, 65, 90), 185);
        assert_eq!(calculate_heart_rate(22, 65, 95), 191);
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    // Input age, "Enter Age: "
    let age: i32 = get_input("Enter Age: ");
    // Input resting_heart_rate, "Enter Resting Heart Rate: "
    let resting_heart_rate: i32 = get_input("Enter Resting Heart Rate: ");

    println!("Intensity     |  Rate");
    println!("--------------|------------");
    // Loop from 55 to 95, increment 5
    for percentage in (55..100).step_by(5) {
        // calculate heart rate
        // display formatted
        let target_heart_rate: i32 = calculate_heart_rate(age, resting_heart_rate, percentage);
        println!("{}            |  {}", percentage, target_heart_rate);
    }
}
