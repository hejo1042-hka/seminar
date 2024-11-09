use proptest::prelude::*;
use seminar::list::List;

proptest! {
    #[test]
    fn prop_removed_item_not_found(
        list_data: Vec<i32>,
        item: i32
    ) {
        let mut list = List::new();
        for x in list_data {
            list.push(x);
        }
        list.push(item);
        list.remove(item);
        let index = list.find(item);

        prop_assert_eq!(index, None);

        // match index {
        //     Some(_) => false,
        //     _ => true,
        // }
    }
}

// fn vec_values_i32() -> impl Strategy<Value = (Vec<i32>)> {
//     let mut result: Vec<i32> = vec![];
//     let usize = rng.gen_range();
//     prop::collection::vec("[0-9]+", 1..100)
//         .prop_flat_map(|vec| {
//
//         })
// }
