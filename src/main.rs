use seminar::list::List;

fn main() {
    println!("Hello, world!");

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
}
