fn main() {
    let a: i8 = -1;
    let b: u8 = 1;
    let c = 1_000_000;
    let d = 0xff;
    let e: u8 = b'A';

    let f: f32 = 1.0;
    let g = 2.0;

    let h = true;
    let i = 'A';

    let t: (i32, f64, char) = (1, 2.0, 'x');
    let (x, y, z) = t;
    println!("t.0 = {}", t.0);

    let unit = ();

    let arr: [i32; 3] = [1, 2, 3];
    let zeros = [0; 5];
    println!("arr[0] = {}", arr[0]);
    println!("zeros = {:?}", zeros);

    let sl = &arr[0..2];
    println!("sl = {:?}", sl);

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    let v2 = vec![1, 2, 3];

    let s1 = String::from("hello");
    let s2: &str = "hello";

    let n: Option<i32> = None;

    println!("f = {}, g = {}", f, g);
    println!("h = {}, i = {}", h, i);
    println!("v = {:?}, v2 = {:?}", v, v2);
    println!("s1 = {}, s2 = {}", s1, s2);
    println!("n = {:?}", n);
}
