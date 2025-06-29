use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Command {
    #[arg(short)]
    range: usize
}

fn get_fibonacci_sequence(range: usize) -> Vec<u128> {
    // Create a vector to store the numbers.
    let mut numbers: Vec<u128> = Vec::new();

    // First two numbers in the sequence are 0 and 1,
    // the rest are the sum of the previous two.
    numbers.push(0);
    numbers.push(1);
    
    // Store the index of the first value
    let mut a: usize = 0;
    // Store the index of the second value
    let mut b: usize = 1;

    // Next we want to push the sum of A and B, and increment the indexes to get the next values
    while numbers.len() < range {
        numbers.push(u128::from(numbers[a] + numbers[b]));
        a = a + 1;
        b = b + 1;
    }

    // Return the sequence
    numbers
}

fn main() {
    // Parse the range input for the sequence.
    let range = Command::parse();
    
    // Get the sequence.
    let numbers = get_fibonacci_sequence(range.range);

    // Print out the sequence.
    println!("{numbers:?}");
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn can_get_first_five_fibonacci_numbers() -> Result<(), ()> {
        // Get the first five fibonacci numbers
        let numbers = get_fibonacci_sequence(5);
        
        assert_eq!(numbers, [0, 1, 1, 2, 3]);
        
        Ok(())
    }

    #[test]
    pub fn can_get_first_ten_fibonacci_numbers() -> Result<(), ()> {
        // Get the sequence containing the first ten numbers
        let numbers = get_fibonacci_sequence(10);

        assert_eq!(numbers, [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);

        Ok(())
    }
}