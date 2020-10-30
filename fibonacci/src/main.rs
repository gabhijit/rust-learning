fn main() {
    let n = 10;
    println!("{}th fibonacci number is {}, fib_loop: {}!", n, fib(n), fib_loop(n));
}

fn fib(i: u32) -> u32 {

    if i > 2 {
        fib(i-1) + fib(i-2)
    } else {
        1
    }
}

fn fib_loop(i: u32) -> u32 {

    let mut last = 0;
    let mut prev = 1;
    let mut count = 1;

    let mut cur = 0;

    if i == 1 || i == 2 {
        return 1
    }

    while count < i {
        cur = prev + last;
        count += 1;
        last = prev;
        prev = cur;
    }

    cur
}
