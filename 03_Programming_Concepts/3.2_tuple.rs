fn main() {
    
    /*
    Tuple can have different types of values.
    Fixed length - cannot grow/shrink in size.
    */

    let tuple: (u32, i64, f64, bool, char) = (656, -854, 6.254, false, 'z');
    
    println!("{}", tuple.0);
    println!("{}", tuple.1);
    println!("{}", tuple.2);
    println!("{}", tuple.3);
    println!("{}", tuple.4);

}