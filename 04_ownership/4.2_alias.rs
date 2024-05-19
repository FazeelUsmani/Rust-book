fn main() {
    let mut v: Vec<i32> = vec![1,2,3];
    let num : &i32 = &v[2];
    v.push(4);
    println!(">>> 3rd ele is {}", *num);
    
    // the issue is that the vector v is both aliased (by the reference num) and
    // mutated (by the operation v.push(4))
}