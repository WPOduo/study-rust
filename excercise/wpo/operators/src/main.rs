fn main() {
    let (a, b) = (7,3);

    println!("{} {} {} {} {}", a + b , a - b, a * b, a / b, a % b);

    println!("{} {}", a > b, a == b);

    println!("{} {} {}", true && false, true || false, !true);

    println!("{} {} {} {}", a & b, a | b, a ^ b, a << 1);

    let mut c = 1;
    c += 2;
    println!("c = {}", c);

    for i in 0..3 {
        print!("{}", i);
    }
    println!();
    for i in 0..=3 {
        print!("{}", i);
    }
    println!();

    let f = a as f64;

    let r = &c;
    let v = *r;
    println!("v = {}", v);
}