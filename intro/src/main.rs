fn main() {
    let x32 = 32;
    let y32 = &x32;
    let z32 = &x32;
    let mut _sum: f64 = 0.0;
    for i in 0..5 {
        let mut x = i as f64;
        _sum += x;
        let even_odd = if i % 2 == 0 { "even" } else { "odd" };
        println!("{} is {}", i, even_odd);
        println!("The sum is {}", _sum);
    }

    let x: i64 = 4;
    println!("{}", x);
    hi(x);
    println!("{}", x);
    let mut array: [i64; 4] = [1, 2, 3, 4];
    for index in 0..4 {
        array[index] = (4 - index) as i64;
        println!("{}", array[index]);
    }

    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let slice2 = &ints;
    let first = ints.get(0);
    let last = ints.get(5);

    println!("first {:?}", first.unwrap());
    println!("last {:?}", last);

    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];
    let maybe_first = v.get(0);

    println!("v is {:?}", v);
    println!("first is {}", first);
    println! {"maybe_first is {:?}", maybe_first};
}

fn hi(mut x: i64) {
    println!("{}", x);
}
