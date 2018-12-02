use std::collections::HashMap;
use std::str::Lines;
use std::cell::RefCell;

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
        process_single_reading(line, &mut frequency);
    }


    frequency
}

fn process_single_reading(line: &str, frequency: &mut i64) -> i64 {
    // Detect polarit
    let number = line.get(1..).expect("bad input");
    let number = number.parse::<i64>().expect("bad convert");
    match line.chars().next() {
        Some('-') => {
            *frequency -= number;
        },
        Some('+') => {
            *frequency += number;
        },
        _ => {
            panic!("bad input");
        }
    }

    *frequency
}

/// Indefinately loop one line over a set of readings,
/// repeating at end of list.
struct ContineousReading<'a> {
    readings: &'a str,
    lines: RefCell<Lines<'a>>,
}

impl<'a> ContineousReading<'a> {
    fn new(readings: &'a str) -> Self {
        Self {
            readings,
            lines: RefCell::new(readings.lines()),
        }
    }

    /// Get the next line of the reading
    fn next(&self) -> &str {
        let next = self.lines.borrow_mut().next();
        match next {
            Some(s) => s,
            None => {
                self.lines.replace(self.readings.lines());
                self.lines.borrow_mut().next().expect("empty input")
            }
        }
    }
}

/// Calibrate the input frequency to the first frequency that occurs twice.
///
/// This will loop the input readings and apply the calibration per reading,
/// until we locate the first frequency that has already occured.
/// This is the calibrated frequency.
fn twice_occurence_calibration(initial: i64, readings: &str) -> i64 {
    let mut occured = HashMap::new();

    let mut frequency: i64 = initial;
    let cont = ContineousReading::new(readings);
    loop {
        let line = cont.next();
        let current = process_single_reading(line, &mut frequency);
        if occured.contains_key(&current) {
            break;
        }
        occured.insert(current, true);
    }

    frequency
}


fn main() {
    // Starting frequency is set to zero.
    {
        let mut frequency: i64 = 0;
        frequency = calibrate_frequency(frequency, INPUT);
        println!("First round frequency: {}", frequency);
    }

    {
        let mut frequency: i64 = 0;
        frequency = twice_occurence_calibration(frequency, INPUT);
        println!("Second round frequency: {}", frequency);
    }
}
