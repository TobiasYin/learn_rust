struct A {
    A: i32,
}

impl std::cmp::PartialOrd for A {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.A.partial_cmp(&other.A)
    }
}

impl std::cmp::PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        self.A == other.A
    }
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let a = Box::new(1);
    println!("{}", a);
}
