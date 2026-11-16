fn main() {
    let mut i = 0;
    let r = loop {
        i += 1;
        if i == 3 { break i * 2; }
    };
    println!("r = {}", r);
 
    let mut j = 0;
    while j < 5 {
        j += 1;
    }
    println!("j = {}", j);

    for x in 0..5 { println!("x = {}", x); }
    for x in [1, 2, 3] { println!("arr val: {}", x); }

    for x in &[1, 2, 3] { println!("ref arr val: {}", x); }

    for (idx, val) in [10, 20].iter().enumerate() {
        println!("idx: {}, val: {}",idx, val);
}

    for i in(0..10).rev().take(3) {
        println!("rev i: {}",i);
}

    for x in 0..10 {
    if x == 1 { continue; }
    if x ==5 { break; }
    println!("ctrl x: {}", x);
    }
    
    'outer: for x in 0..3 {
        for y in 0..3 {
            if x + y == 3 { break 'outer; }
            println!("outer x: {}, y: {}", x, y);
    }
}

    let mut st = Some(1);
    while let Some(v) = st {
        println!("st v: {}", v);
        st = None;
    }
}

