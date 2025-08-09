fn main() {
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("Value of number is: {number}");

    // let mut counter = 0;
    //
    // let result = loop {
    //     counter += 1;
    //
    //     if counter == 10 {
    //         break counter * 2; // the value after the 'break' will be returned
    //     }
    // };
    //
    // println!("The result is {result}");

    let mut count = 0;

    'counting_up: loop { // naming the loop
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // break the inner loop
            }

            if count == 2 {
                break 'counting_up; // break external loop
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("Value is: {element}");
    }

    for number in (1..4).rev() {
        // a range from start to end-1, 'rev()' inverts the range
        println!("{number}!");
    }
    println!("LIFTOFF!!!!");
}
