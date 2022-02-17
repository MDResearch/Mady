fn withoutsort() {
    let mut v = vec![0; 100];
    for i in &mut v {
        *i = fastrand::i32(..);
    }
    let b = fastrand::i32(..);
    for i in v {
        if i > b {
            print!("{},", i);
        }
    }
}
fn withsort() {
    let mut v = vec![0; 100];
    for i in &mut v {
        *i = fastrand::i32(..);
    }
    let b = fastrand::i32(..);
    v.sort();
    for i in v {
        if i > b {
            print!("{},", i);
        }
    }
}
