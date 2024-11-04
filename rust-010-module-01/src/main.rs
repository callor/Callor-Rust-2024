mod front_of_house;
pub use crate::front_of_house::hosting;

fn main() {
    println!("Hello, world!");
    front_of_house::hosting::add_to_waitlist();
}
