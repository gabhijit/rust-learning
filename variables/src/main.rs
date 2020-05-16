fn main() {
    let mut x = 5;
    println!("The value of x is {}!", x);
    x = 6;
    println!("The value of x is {}!", x);


    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is {}!", spaces);


    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is {}!", guess);


    let tup = (100, 2.0, 1);
    println!("The value of First element of tuple is {}!", tup.0);

    let arr: [i32; 5] = [-1, 0, 1, 2, 3];
    println!("The value of 3rd element of arr is {}!", arr[2]);
}
