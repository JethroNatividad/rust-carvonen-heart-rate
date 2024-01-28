// A program that calculates your heart rate from 55% to 95%.
// Input: Age, Resting heart rate
// Process: loop from 55 - 95, increment by 5. Use carvonen formula to calculate heart rate.
// Output: The intensity percentage and heart rate

#[cfg(test)]
mod tests {
    use super::*;

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
fn main() {
    println!("Hello, world!");
}