use digit::Digit;
use std::cmp;

type Carry = Digit;

#[derive(Copy, Clone, Debug)]
struct AdderResult(Digit, Carry);

impl cmp::PartialEq for AdderResult {
    fn eq(&self, other: &AdderResult) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

fn single_digit_half_adder(a: &Digit, b: &Digit) -> AdderResult {
    let s: Digit = *a ^ *b;
    let c: Carry = *a & *b;
    AdderResult(s, c)
}

fn single_digit_adder(a: &Digit, b: &Digit, c_in: &Carry) -> AdderResult {
    let s: Digit = *a ^ *b ^ *c_in;
    let c_out = (*a & *b) | (*a & *c_in) | (*b & *c_in);
    AdderResult(s, c_out)
}

fn four_digits_adder(a: &[Digit], b: &[Digit]) -> [AdderResult; 4] {
    let mut results = [AdderResult(Digit('0'), Digit('0')); 4];
    for i in 0..4 {
        match i {
            0 => {
                let r: AdderResult = single_digit_half_adder(&a[i], &b[i]);
                results[0] = r;
            },
            1...3 => {
                let r: AdderResult = single_digit_adder(&a[i], &b[i], &results[i-1].1);
                results[i] = r;
            },
            _ => panic!("unexpected error")
        }
    }
    results
}

#[test]
fn half_adder_test() {
    let (one, zero) = (Digit('1'), Digit('0'));

    let r: AdderResult = single_digit_half_adder(&zero, &zero);
    let d: Digit = Digit('0');
    let c: Carry = Digit('0');
    assert_eq!((r.0, r.1), (d, c));
    
    let r: AdderResult = single_digit_half_adder(&zero, &one);
    let d: Digit = Digit('1');
    let c: Carry = Digit('0');
    assert_eq!((r.0, r.1), (d, c));

    let r: AdderResult = single_digit_half_adder(&one, &zero);
    let d: Digit = Digit('1');
    let c: Carry = Digit('0');
    assert_eq!((r.0, r.1), (d, c));

    let r: AdderResult = single_digit_half_adder(&one, &one);
    let d: Digit = Digit('0');
    let c: Carry = Digit('1');
    assert_eq!((r.0, r.1), (d, c));
}

#[test]
fn adder_test() {
    let (one, zero) = (Digit('1'), Digit('0'));
    let (c_in_zero, c_in_one) = (Digit('0'), Digit('1'));

    let r: AdderResult = single_digit_adder(&zero, &zero, &c_in_zero);
    assert_eq!((r.0, r.1), (zero, zero));

    let r: AdderResult = single_digit_adder(&zero, &zero, &c_in_one);
    assert_eq!((r.0, r.1), (one, zero));

    let r: AdderResult = single_digit_adder(&zero, &one, &c_in_zero);
    assert_eq!((r.0, r.1), (one, zero));

    let r: AdderResult = single_digit_adder(&zero, &one, &c_in_one);
    assert_eq!((r.0, r.1), (zero, one));

    let r: AdderResult = single_digit_adder(&one, &zero, &c_in_zero);
    assert_eq!((r.0, r.1), (one, zero));

    let r: AdderResult = single_digit_adder(&one, &zero, &c_in_one);
    assert_eq!((r.0, r.1), (zero, one));

    let r: AdderResult = single_digit_adder(&one, &one, &c_in_zero);
    assert_eq!((r.0, r.1), (zero, one));

    let r: AdderResult = single_digit_adder(&one, &one, &c_in_one);
    assert_eq!((r.0, r.1), (one, one));
}

#[test]
fn four_digits_adder_test() {
    let (one, zero) = (Digit('1'), Digit('0'));

    let a_digits = vec![zero, zero, one, zero];
    let b_digits = vec![zero, zero, zero, one];
    let results = four_digits_adder(&a_digits[..], &b_digits[..]);
    let test = [
        AdderResult(zero, zero),
        AdderResult(zero, zero),
        AdderResult(one, zero),
        AdderResult(one, zero)
    ];
    assert_eq!(results, test);
}