fn main() {
    for n in 1..10 {
        println!("{}", fibonacci_number(n));
    }
}

fn fibonacci_number(n: i32) -> i32 {
    if n == 1 || n == 2{
        return 1;
    } else {
        return fibonacci_number(n-1) + fibonacci_number(n-2);
    }
}
