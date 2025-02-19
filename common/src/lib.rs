pub use paste::paste;

#[macro_export]
macro_rules! advent_of_rust {
    ($($day:expr => $part_1:expr $(, $part_2:expr)?);* $(;)?) => {
        use common::paste;

        paste! {
            $(mod [<day_ $day>];)*

            pub fn ignore_clippy_unused_errors() {
                $(
                    let _ = [<day_ $day>]::part_1;
                    $(
                        let _ = [<day_ $day>]::part_2;
                        let _ = $part_2;
                    )?
                )*
            }

            mod tests {
                use super::*;
                $(
                    const [<DAY_ $day _INPUT>]: &str = include_str!(concat!("day_", $day, ".txt"));

                    #[test]
                    fn [<day_ $day _part1>]() {
                        assert_eq!([<day_ $day>]::part_1([<DAY_ $day _INPUT>]), $part_1);
                    }

                    $(
                        #[test]
                        fn [<day_ $day _part2>]() {
                            assert_eq!([<day_ $day>]::part_2([<DAY_ $day _INPUT>]), $part_2);
                        }
                    )?
                )*
            }
        }
    };
}
