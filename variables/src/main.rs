/*fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
}*/

fn main() {
    let x = 4;
    let x = x + 1;
    {
        let x = x + 1;
        println!("The value of x is {x}");
    }
    println!("The value of x is {x}");
}
