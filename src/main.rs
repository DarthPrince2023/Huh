use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Command {
    #[arg(short)]
    range: usize
}

fn main() {
    let mut numbers: Vec<u128> = Vec::new();
    numbers.push(0);
    numbers.push(1);
    
    let range = Command::parse();
    let mut a: usize = 0;
    let mut b: usize = 1;
    while numbers.len() < range.range {
        numbers.push(u128::from(numbers[a] + numbers[b]));
        a = a + 1;
        b = b + 1;
    }
    println!("{numbers:?}");
}
