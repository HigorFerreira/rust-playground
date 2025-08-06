fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 102, 101, 120, 65];
    println!("The largest number is {}", largest(&number_list));

    let number_list = vec!['H', 'i', 'g', 'o', 'r'];
    println!("The largest char is {}", largest(&number_list));
}