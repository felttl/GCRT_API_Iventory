
use rand::RngExt;


mod Inventory;

fn main() {
    let min: i8 = 5;
    let max: i8 = 50;
    let val: i8 = rand::rng().random_range(min..=max);
    println!("random number between [{min},{max}] is : {val}");
    println!("Hello, world!");
}
