fn fahrenheit_to_celcius(f: f64) -> f64 {
    (f-32.0)*(5.0/9.0)
}

fn nth_fibonacci(num: u128) -> u128 {
    let mut a = 0;
    let mut b = 1;
    if num == 0 {
        return a; 
    }
    if num == 1 {
        return b;
    }
    
    // let mut _c = 0;
    let mut cnt = 1;
    while cnt < num {
        let mut _c = a+b;
        a = b;
        b = _c;
        cnt += 1;
    }
    b
}

fn main() {

    println!("Fahrenheit to Celcius = {}", fahrenheit_to_celcius(212.0));
    println!("Nth Fibonacci = {}", nth_fibonacci(10));
   
}