use proptest::collection::vec;
use proptest::prelude::*;
use seminar::list::List;
use proptest::strategy::Strategy;
use proptest::prelude::{any, proptest};

// proptest! {
//     #[test]
//     fn prop_removed_item_not_found_large_space(
//         list_data: Vec<i32>,
//         // list_data in vec(0..30, 1..100),
//         item: i32,
//         // item in 0..30,
//     ) {
//         let mut list = List::new();
//         for x in list_data {
//             list.push(x);
//         }
//         // list.push(item);
//         list.remove(item);
//         let index = list.find(item);
//
//         prop_assert_eq!(index, None);
//     }
// }

fn own_list_large() -> impl Strategy<Value = List<i32>> {
    vec(any::<i32>(), 1..100)
        .prop_map(|x| {
            let mut list = List::new();
            for item in x {
                list.push(item);
            }
            list
        })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100000))]
    #[test]
    fn prop_removed_item_not_found_large(
        mut list in own_list_large(),
        item in any::<i32>()
    ) {
        // list.push(item); // PreCondition
        list.remove(item);
        let index = list.find(item);

        prop_assert_eq!(index, None);
    }
}

fn own_list() -> impl Strategy<Value = List<i32>> {
    vec(0..30, 1..100)
        .prop_map(|x| {
            let mut list = List::new();
            for item in x {
                list.push(item);
            }
            list
        })
}

proptest! {
    #[test]
    fn prop_removed_item_not_found(
        mut list in own_list(),
        item in 0..30
    ) {
        list.remove(item);
        let index = list.find(item);

        prop_assert_eq!(index, None);
    }
}

proptest! {
    #[test]
    fn prop_size_reduced(
        mut list in own_list(),
        item in 0..30
    ) {
        list.push(item);
        let size_before_remove = list.length();
        list.remove(item);
        let size_after = list.length();

        prop_assert!(size_before_remove > size_after);
    }
}

proptest! {
    #[test]
    fn prop_size_reduced_by_num_occurences(
        mut list in own_list(),
        item in 0..30
    ) {
        let orgiginal_size = list.length();
        let mut number_occurences = 0;
        for x in &list {
            if x == item {
                number_occurences += 1;
            }
        }
        list.remove(item);
        let size_after = list.length();

        prop_assert_eq!(orgiginal_size, size_after + number_occurences);
    }
}
