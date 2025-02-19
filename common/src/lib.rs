pub use paste::paste;

#[macro_export]
macro_rules! advent_of_rust {
    ($($day:expr => $part_1:expr, $part_2:expr);* $(;)?) => {
        use common::paste;
        paste! {
            $(mod [<day_ $day>];)*
            use common::Solution;

            mod tests {
                use super::*;
                $(
                    #[test]
                    fn [<day_ $day>]() {
                        let input = include_str!(concat!("day_", $day, ".txt"));

                        assert_eq!([<day_ $day>]::part_1(input), $part_1);
                        assert_eq!([<day_ $day>]::part_2(input), $part_2);
                    }
                )*
            }
        }
    };
}
