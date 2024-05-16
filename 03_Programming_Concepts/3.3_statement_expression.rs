fn main() {
    let y = {          // Expression returns a value, here it's 4.
        let x = 3;     // statement are instructions and doesn't return a value.
        x + 1
    };

    println!("The value of y is: {y}");
}