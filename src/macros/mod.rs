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
