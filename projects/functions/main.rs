fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
fn plus_one_with_return(x: i32) -> i32 {
    return x + 1
}

fn main() {
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 5;
        x+1
    };

    println!("y is {y}");

    let p1 = plus_one(5);
    let p2 = plus_one_with_return(6);
    println!("Plus one: {p1}");
    println!("Plus one with return: {p2}");
}
