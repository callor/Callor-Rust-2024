
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);
    println!("The value of x is: {}",x.1);

    let tup  = (1,2,3,4,5);
    println!("The value of tup is: {:?}", tup);
}
