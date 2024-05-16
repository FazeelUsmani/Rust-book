fn main() {
    let arr = [1,2,3,4,5];
    for ele in arr {
        print!("{ele} ");
    }
    println!();
    
    // Reversing the range from 1 to 4
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
