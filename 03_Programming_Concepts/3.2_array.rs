fn main() {
    
    /*
        Array contains collection of values of same datatype.
        Fixed length. Vectors are of dynamic length. 
        Stores data in stack rather than heap memory.
    */
    
    // Here, I'm preceding days variable with '_' to avoid Unused variable warning.
    let _days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    
    let digits: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    let _five_twice = [5; 2];          // five = [5, 5]
    
    // Accessing is index 0 based
    let zero = digits[0];
    let one = digits[1];
    let index = digits[9];
    
    println!("{} {} {}", zero, one, index);
    
}