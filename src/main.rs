use proc_macros::print_tree_macro;

#[macro_use]
mod average;
#[macro_use]
mod math;

fn main() {
    print_tree_macro!( pub struct SomeStruct<T> {
        pub value: T,
        pub another_value: i32,
    };

    impl<T> SomeStruct<T> {

        }
    );
    println!("{} {} {}", average!(2, 3, 4), average!(1, 2, 3), average!(1, 2, 2, 4, 5));
    println!("{} {} {}", min!(1, 3, 2), min!(0.1, 0.3, 0.4), min!(-1, 0, 3));
    println!("{} {} {}", max!(3, 2, 3), max!(32, 32, 31), max!(0.1, 0.3, 0.5));
}