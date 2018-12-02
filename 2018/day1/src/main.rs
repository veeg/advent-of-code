
const INPUT: &str = include_str!("../input/data");

/// Calibrate the initial input frequency based on the newline seperated
/// reading instructions.
///
/// Each instruction is prefixed by a positive('+') or a negative('-')
/// sign, indicating the calibration step to perform.
///
/// Panics on bad input data. Use only in experimental space flights,
/// not for passenger vehicles.
fn calibrate_frequency(initial: i64, readings: &str) -> i64 {
    let mut frequency: i64 = initial;

    for line in readings.lines() {
        // Detect polarit
        let number = line.get(1..).expect("bad input");
        let number = number.parse::<i64>().expect("bad convert");
        match line.chars().next() {
            Some('-') => {
                frequency -= number;
            },
            Some('+') => {
                frequency += number;
            },
            _ => {
                panic!("bad input");
            }
        }
    }


    frequency
}

fn main() {
    // Starting frequency is set to zero.
    let mut frequency: i64 = 0;

    println!("Starting frequency: {}", frequency);

    frequency = calibrate_frequency(frequency, INPUT);

    println!("End frequency: {}", frequency);
}
