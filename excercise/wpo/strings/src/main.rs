fn main() {
    let c1 = 'A';
    let c2 = '中';
    let c3 = '😊';
    println!("c1={}, c2={}, c3={}", c1, c2, c3);
    println!("emoji len: {}", '😊'.len_utf8());

    let s1: &str = "hello";

    let mut s2 = String::from("hello");
    s2.push_str(", world");
    s2.push('!');

    let s3: String = s1.to_string();
    let s4: &str = &s2;

    let s5 = String::from("a") + &s2;
    let s6 = format!("{}-{}", s1, s2);

    println!("s1={}", s1);
    println!("s2={}", s2);
    println!("s3={}", s3);
    println!("s4={}", s4);
    println!("s5={}", s5);
    println!("s6={}", s6);

    for ch in "hi中".chars() {println!("char: {}", ch); }
    for b in "hi".bytes() { println!("byte: {}", b); }

    println!("{}", "中".len());
    println!("{}", "中".chars().count());

}
