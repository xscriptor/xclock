pub const WIDTH: usize = 5;
pub const HEIGHT: usize = 5;

// 0-9, colon
// true represents a filled block
pub const FONT: [[[bool; WIDTH]; HEIGHT]; 11] = [
    // 0
    [
        [true, true, true, true, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, true, true, true, true],
    ],
    // 1
    [
        [false, false, false, false, true],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [false, false, false, false, true],
    ],
    // 2
    [
        [true, true, true, true, true],
        [false, false, false, false, true],
        [true, true, true, true, true],
        [true, false, false, false, false],
        [true, true, true, true, true],
    ],
    // 3
    [
        [true, true, true, true, true],
        [false, false, false, false, true],
        [true, true, true, true, true],
        [false, false, false, false, true],
        [true, true, true, true, true],
    ],
    // 4
    [
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, true, true, true, true],
        [false, false, false, false, true],
        [false, false, false, false, true],
    ],
    // 5
    [
        [true, true, true, true, true],
        [true, false, false, false, false],
        [true, true, true, true, true],
        [false, false, false, false, true],
        [true, true, true, true, true],
    ],
    // 6
    [
        [true, true, true, true, true],
        [true, false, false, false, false],
        [true, true, true, true, true],
        [true, false, false, false, true],
        [true, true, true, true, true],
    ],
    // 7
    [
        [true, true, true, true, true],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [false, false, false, false, true],
    ],
    // 8
    [
        [true, true, true, true, true],
        [true, false, false, false, true],
        [true, true, true, true, true],
        [true, false, false, false, true],
        [true, true, true, true, true],
    ],
    // 9
    [
        [true, true, true, true, true],
        [true, false, false, false, true],
        [true, true, true, true, true],
        [false, false, false, false, true],
        [true, true, true, true, true],
    ],
    // : (10)
    [
        [false, false, false, false, false],
        [false, false, true, false, false],
        [false, false, false, false, false],
        [false, false, true, false, false],
        [false, false, false, false, false],
    ],
];

pub fn get_digit(n: usize) -> &'static [[bool; WIDTH]; HEIGHT] {
    if n > 10 {
        &FONT[0]
    } else {
        &FONT[n]
    }
}
