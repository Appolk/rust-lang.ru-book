use std::io;
fn main() {
   //Generate the nth Fibonacci number.
    println!("input n:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("exception");
    println!("{}",fibonacci(buffer.trim().parse().expect("not integer")));
}
fn fibonacci(n:usize)->usize{

    if n <=1{
       return n;
    }
    let mut prev:usize = 0;
    let mut curr:usize = 1;

    for i in 2..=n {
    let next = prev + curr;
    println!("{}. {} + {} -> {}",i-2,prev,curr,next);
    prev = curr;
    curr = next;
    }
    curr
}