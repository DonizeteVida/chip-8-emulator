#[macro_export]
macro_rules! nibbles {
    ($($n:tt),*) => {
        ( $(nibbles!(@map $n)),* )
    };

    (@map A) => (0xau8);
    (@map B) => (0xbu8);
    (@map C) => (0xcu8);
    (@map D) => (0xdu8);
    (@map E) => (0xeu8);
    (@map F) => (0xfu8);
    (@map 0) => (0x0u8);
    (@map 1) => (0x1u8);
    (@map 2) => (0x2u8);
    (@map 3) => (0x3u8);
    (@map 4) => (0x4u8);
    (@map 5) => (0x5u8);
    (@map 6) => (0x6u8);
    (@map 7) => (0x7u8);
    (@map 8) => (0x8u8);
    (@map 9) => (0x9u8);
    (@map _) => (_)
}

#[macro_export]
macro_rules! count {
    () => (0);
    ($a:expr $(, $b:expr)*) => {
        1 + count!($($b),*)
    };
}

#[macro_export]
macro_rules! joinibble {
    () => {0};

    ($head:expr $(, $tail:expr)*) => {
        (($head as u16) << (4 * count!($($tail),*))) | joinibble!($($tail),*)
    };

    ($a:tt $b:tt) => ($a << 4 | $b)
}

#[cfg(test)]
mod count_tests {
    #[test]
    fn can_count() {
        let result = count!(1, 2, 3, 4);
        assert_eq!(result, 4);
    }
}

#[cfg(test)]
mod joinibble_tests {
    #[test]
    fn can_join_one() {
        let result = joinibble!(0xA);
        assert_eq!(result, 0xA);
    }

    #[test]
    fn can_join_two() {
        let result = joinibble!(0xF, 0xD);
        assert_eq!(result, 0xFD);
    }

    #[test]
    fn can_join_three() {
        let result = joinibble!(0xA, 0xB, 0xC);
        assert_eq!(result, 0xABC);
    }

    #[test]
    fn can_join_four() {
        let result = joinibble!(0xA, 0xB, 0xC, 0xD);
        assert_eq!(result, 0xABCD);
    }

    #[test]
    fn no_comma() {
        let result = joinibble!(0xF 0xD);
        assert_eq!(result, 0xFD);
    }
}
