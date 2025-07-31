fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 3, 3];

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);
}
