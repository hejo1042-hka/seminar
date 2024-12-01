use seminar::addition::add_two_numbers;
use seminar::list::List;

fn main() {
    println!("Hello, list example!");
    let mut list = List::new();
    list.push(10);
    list.push(20);
    list.push(30);
    list.push(40);
    println!("Original List: {:?}", list);
    list.remove(30);
    println!("After removal: {:?}", list);
    list.remove(100);
    println!("After attempting to remove 100: {:?}", list);
    println!("find index of 10: {:?}", list.find(10));
    println!("find index of 30 after removal: {:?}", list.find(30));

    println!("--------------------------------------------------------");

    println!("Hello, addition example!");
    let number1 = 12;
    let number2 = 34;
    println!("adding {:?} and {:?}", number1, number2);
    let result = add_two_numbers(number1, number2);
    println!("result is: {:?}", result);

    // Raises an error, because this is an addition with overflow.
    // println!("Hello, addition example 2.0!");
    // let number1 = 656391597;
    // let number2 = 3638575699;
    // println!("adding {:?} and {:?}", number1, number2);
    // let result = add_two_numbers(number1, number2);
    // println!("result is: {:?}", result);
}
