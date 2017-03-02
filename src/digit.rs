use std::ops;
use std::cmp;

#[derive(Copy, Clone, Debug)]
pub struct Digit(pub char);

impl ops::BitAnd for Digit {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        match self.0 {
            '0' => Digit('0'),
            _ => {
                match rhs.0 {
                    '0' => Digit('0'),
                    _ => Digit('1')
                }
            }
        }
    }
}

impl ops::BitOr for Digit {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        match self.0 {
            '0' => {
                match rhs.0 {
                    '0' => Digit('0'),
                    _ => Digit('1')
                }
            },
            _ => Digit('1')
        }
    }
}

impl ops::BitXor for Digit {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        match self.0 {
            '0' => {
                match rhs.0 {
                    '0' => Digit('0'),
                    _ => Digit('1')
                }
            },
            _ => {
                match rhs.0 {
                    '0' => Digit('1'),
                    _ => Digit('0')
                }
            }
        }
    }
}

impl ops::Not for Digit {
    type Output = Self;
    fn not(self) -> Self {
        match self.0 {
            '0' => Digit('1'),
            _ => Digit('0')
        }
    }
}

impl cmp::PartialEq for Digit {
    fn eq(&self, other: &Digit) -> bool {
        self.0 == other.0
    }
}

#[test]
fn digit_and_test() {
    let (zero, one) = (Digit('0'), Digit('1'));

    assert_eq!((zero & zero).0, '0');
    assert_eq!((zero & one).0, '0');
    assert_eq!((one & zero).0, '0');
    assert_eq!((one & one).0, '1');
}

#[test]
fn digit_or_test() {
    let (zero, one) = (Digit('0'), Digit('1'));

    assert_eq!((zero | zero).0, '0');
    assert_eq!((zero | one).0, '1');
    assert_eq!((one | zero).0, '1');
    assert_eq!((one | one).0, '1');
}

#[test]
fn digit_xor_test() {
    let (zero, one) = (Digit('0'), Digit('1'));

    assert_eq!((zero ^ zero).0, '0');
    assert_eq!((zero ^ one).0, '1');
    assert_eq!((one ^ zero).0, '1');
    assert_eq!((one ^ one).0, '0');
}

#[test]
fn digit_not_test() {
    let (zero, one) = (Digit('0'), Digit('1'));

    assert_eq!((!zero).0, '1');
    assert_eq!((!one).0, '0');
}