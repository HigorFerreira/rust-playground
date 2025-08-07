fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 102, 101, 120, 65];
    println!("The largest number is {}", largest(&number_list));

    let number_list = vec!['H', 'i', 'g', 'o', 'r'];
    println!("The largest char is {}", largest(&number_list));

    let p = Point { x: 1.0, y: 2.2 };
    p.distance_from_origin();
}