fn main() {
    let mut x: Box<i32> = Box::new(1);
    println!("{}", x);
    
    let a: i32 = *x;
    println!("{} {}", x, a);
    *x += 1;
    println!("{} {}", x, a);
    
    let r1: &Box<i32> = &x;
    println!("{} {} {}", x, a, r1);
    
    let b: i32 = **r1;
    println!("{} {} {} {}", x, a, r1, b);
    
    let r2: &i32 = &*x;
    println!("{} {} {} {} {}", x, a, r1, b, r2);
    
    let c: i32 = *r2;
    println!("{} {} {} {} {} {}", x, a, r1, b, r2, c);
    
    println!("{}", &r1);
}