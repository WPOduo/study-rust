fn main() {
    let n = 5;
    if n > 0{
        println!("positive");
    } else if n < 0{
        println!("negative");
    } else {
        println!("zero");
    }
    let label = if n % 2 == 0 { "even" } else { "odd" };
    if n > 0 { }
    match n {
        0 => println!("zero"),
        1 | 2 => println!("one or two"),
        3..=9 => println!("3 to 9"),
        _ => println!("other"),
    }
    let s = match n { 0 => "zero", _ => "nonzero" };
    match n {
        x if x < 0 => println!("neg"),
        _ => println!("non-neg"),
    }
    let opt: Option<i32> = Some(3);
    if let Some (v) = opt { println!("{}", v); }
    let Some(v) = opt else { return; };
}
