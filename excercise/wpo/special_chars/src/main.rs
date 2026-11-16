fn main() {

    let n1 = "\n";
    let tb = "\t";
    let cr = "\r";
    let bs = "\\";
    let q = "\"";
    let sq = '\'';
    let z = "\0";

    let u1 = "\u{4E2D}";
    let e1 = "\u{1F600}";

    let raw = r"C: \Users\name";
    let raw2 = r#"He said "hi""#;
    let raw3 = r##"a "#" b"##;

    let b: &[u8; 2] = b"hi";
    let braw = br"raw\nbytes";

    let long = "abc\
                def";

    println!("q = {}", q);
    println!("sq = {}", sq);
    println!("bs = {}", bs);
    println!("u1 = {}", u1);
    println!("e1 = {}", e1);
    println!("raw = {}", raw);
    println!("raw2 = {}", raw2);
    println!("raw3 = {}", raw3);
    println!("b = {:?}", b);
    println!("braw = {:?}", braw);
    println!("long = {}", long);
    println!("z = {}", z);
}