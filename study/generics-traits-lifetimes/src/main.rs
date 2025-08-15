// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//
//     for v in list {
//         if v > largest {
//             largest = v;
//         }
//     }
//     largest
// }

// struct Point<T,U> {
//     x: T,
//     y: U,
// }
// impl<T,U> Point<T,U> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
// impl Point<f32, f32> { // only applies to T and U beeing f32
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }


struct Point<X1,Y1> {
    x: X1,
    y: Y1,
}

impl<X1,Y1> Point<X1,Y1> {
    // get the y of another Point and mix with the current Point
    fn mixup<X2,Y2> (self, other: Point<X2,Y2>) -> Point<X1,Y2> {
        Point { x: self.x, y: other.y }
    }
}

enum Option<T> {
    Some(T),
    None,
}
enum Result<T,E> {
    Ok(T),
    Err(E),
}

fn main() {
    let p1 = Point{ x: 5, y: 10.4 };
    let p2 = Point{ x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}\np3.y = {}", p3.x, p3.y); // 5 'c'


    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("Largest number: {result}");
    //
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("Largest char: {result}");

    let integer = Point{ x: 5, y: 10 };
    let float = Point{ x: 1.0, y: 4.0 };
    let integer_float = Point{ x: 5, y: 4.0 };

    let p = Point{ x: 5, y: 10 }; 
    println!("p.x: {}", p.x);
}
