#[macro_use] mod average;

fn main() {
    println!("{} {}", average!(2, 3, 4), average!(1, 2, 3));
}