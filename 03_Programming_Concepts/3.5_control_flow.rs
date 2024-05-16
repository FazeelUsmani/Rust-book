fn is_even(x: u32) -> bool{
    // If condition
    if (x%2) == 1 {
        return false;
    }
    true
}

fn break_value_return(mut counter: u64) -> u64 {
    /*
        Using mut as we're changing counter value in this function scope.
        So, a new memory is allocated to counter variable, it won't use the
        main fn counter memory as we're passing by value not by reference.
    */
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    result
}
fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
/*
    Condition must contain bool expressions only, integers doesn't work.
    For example: if 1 {/* */} will throw an error. 
                 if (x-2) == 0 {/* */} will compile as this expression evaluates
                 to either true or false depending on the x value.
*/

fn main() {
    let x: u32 = 21+9;
    
    // If Condition
    println!("{x} is even: {}", {is_even(x)});
    
    // Here make sure that the if and else return values of same type i.e., bool
    let is_even_var: bool = if (x%2) == 1 {false} else {true};
    println!("{x} is even: {is_even_var}");
    
    // Doesn't compile
    // let z = 1;
    // if z {
    //     println!("ahdlkfa");
    // }
    
    // Unlimited loop until you stop.
    // loop {
    //     println!("Press Ctrl^C  to stop!!");
    // }
    
    // break - returning break value
    let counter: u64 = 0;
    /*
        Here, why counter is not mut?
        Because in this scope we're not changing counter value. It is 
        changed in the function break_value_return. If you try to print counter 
        value after calling this function, it'll return 0 because in this main fn
        scope it's 0
    */
    let result = break_value_return(counter);
    println!("The break return value is {result}");
    println!("counter value after function evaluation is {counter}");
    println!("------------------------------");
    
    // Loop labels to differentiate between multiple loops
    loop_labels();
    println!("------------------------------");
}