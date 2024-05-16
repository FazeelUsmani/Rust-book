fn area_of_rectangle(length: u32, breadth: u32) -> u64 {
    /* Using into() to convert u32 * u32 into u64 as multiplying 
       two 32-bit numbers can result upto a 64-bit number.
    */
    return (length * breadth).into();
}

fn main() {
    let length:u32 = 5;
    let breadth: u32 = 6;
    
    let area: u64 = area_of_rectangle(length, breadth);
    println!("Area of rectangle of {length} length and {breadth} breadth is {area}");
}