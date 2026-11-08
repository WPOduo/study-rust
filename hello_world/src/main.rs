fn main() {
    let c1 = 'A';
    let c2 = '中';
    let c3 = '😃';
    println!("{}", '😃'.len_utf8());
    let s1: &str = "hello";
    let mut s2 = String::from("hello");
    s2.push_str(",world");
    s2.push('!');
    let s3: String = s1.to_string();
    let s4: &str = &s2;
    let s5 = String::from("a") + &s2;
    let s6 = format!("{}-{}", s1, s2);
    for ch in "hi中".chars() { }
    for b in "hi".bytes() { }
    println!("{}", "中".len());
    println!("{}", "中".chars().count());
}

























