use quickcheck_macros::quickcheck;
use seminar::list::List;

#[quickcheck]
fn prop_removed_item_not_found(list_data: Vec<i32>, item: i32) -> bool {
    let mut list = List::new();
    for x in list_data {
        list.push(x);
    }
    list.remove(item);
    let index = list.find(item);

    match index {
        Some(_) => false,
        _ => true,
    }
}

#[quickcheck]
fn prop_remove_decreases_size(list_data: Vec<i32>, item: i32) -> bool {
    let mut list = List::new();
    let mut occurrences: usize = 0;
    for x in list_data {
        list.push(x);
        if x == item {
            occurrences += 1;
        }
    }
    let original_len = list.length();
    list.remove(item);

    list.length() == original_len - occurrences
}
