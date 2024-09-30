use create::garden::vegetables::Asparagus;

// includes all code in garden.rs
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
