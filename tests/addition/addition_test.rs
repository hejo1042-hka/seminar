use proptest::prelude::*;
use proptest::prelude::{any, proptest};
use seminar::addition::add_two_numbers;

proptest! {
    #[test]
    fn prop_kommutativität(
        number1 in any::<i32>(),
        number2 in any::<i32>(),
    ) {
        let ab = add_two_numbers(number1, number2);
        let ba = add_two_numbers(number2, number1);

        prop_assert_eq!(ab, ba);
    }
}

proptest! {
    #[test]
    fn prop_kommutativität2(
        number1: i32,
        number2: i32,
    ) {
        let ab = add_two_numbers(number1, number2);
        let ba = add_two_numbers(number2, number1);

        prop_assert_eq!(ab, ba);
    }
}

proptest! {
    #[test]
    fn prop_neutrales_element(
        number1: i32,
    ) {
        let result = add_two_numbers(number1, 0);

        prop_assert_eq!(result, number1);
    }
}
