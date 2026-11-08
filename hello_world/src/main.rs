fn main() {
    let mut i = 0;
    let r = loop {
        i += 1;
        if i == 3 { break i * 2; } 
    };
    while i < 5 {
        i += 1;
    }
    for x in 0..5 { println!("{}", x); }
    for x in [1, 2, 3] { }
    for x in &[1, 2, 3] { }
    for (idx, val) in [10, 20].iter().enumerate() { }
    for i in (0..10).rev() { }
    for x in 0..10 {
        if x == 1 { continue; }
        if x == 5 { break; }
    }
    'outer: for x in 0..3 {
        for y in 0..3 {
            if x + y == 3 { break 'outer; }
        }
    }
    let mut st = Some(1);
    while let Some(v) = st { st = None; }

    
}
