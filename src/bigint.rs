/// BigInt
/// A struct that represents a large integer
/// 
/// ```
/// | sign |       zeros         |           digits              |  size  |
/// +------+---------------------+--------+--------+-----+-------+--------+
/// |  +-  |   0   |   0   | ... | first  | second | ... | last  |        |
/// +------+---------------------+--------+--------+-----+-------+--------+
/// | bool |  u8   |  u8   | ... |  u8    |  u8    | ... |  u8   |   u64  |
/// ```
#[derive(Debug, Clone, Copy)]
pub struct BigInt<const MAX_SIZE: usize> {
    is_negative: bool,
    digits: [u8; MAX_SIZE],
    current_size: usize,
}


impl<const MAX_SIZE: usize> std::ops::Add for BigInt<MAX_SIZE> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.add(other)
    }
}

impl<const MAX_SIZE: usize> std::ops::Sub for BigInt<MAX_SIZE> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.sub(other)
    }
}

impl<const MAX_SIZE: usize> std::ops::Mul for BigInt<MAX_SIZE> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.mul(other)
    }
}

impl<const MAX_SIZE: usize> std::ops::Div for BigInt<MAX_SIZE> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self.div(other).0
    }
}

impl<const MAX_SIZE: usize> std::ops::Rem for BigInt<MAX_SIZE> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        self.div(other).1
    }
}

impl<const MAX_SIZE: usize> std::cmp::PartialEq for BigInt<MAX_SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.equal(*other)
    }
}

impl<const MAX_SIZE: usize> std::cmp::PartialOrd for BigInt<MAX_SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.less(*other) {
            Some(std::cmp::Ordering::Less)
        } else if self.equal(*other) {
            Some(std::cmp::Ordering::Equal)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}



impl<const MAX_SIZE: usize> std::fmt::Display for BigInt<MAX_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::with_capacity(self.current_size + 1);
        if self.is_negative {
            result.push('-');
        }
        let mut i = MAX_SIZE - self.current_size;
        while i < MAX_SIZE {
            result.push((self.digits[i] + '0' as u8) as char);
            i += 1;
        }
        write!(f, "{}", result)
    }
}

impl<const MAX_SIZE: usize> BigInt<MAX_SIZE> {
    pub const DEFAULT: Self = Self { is_negative: false, digits: [0; MAX_SIZE], current_size: 1};

    pub const fn from_str(s: &str) -> Self {
        let mut result = Self::DEFAULT;

        let chars = s.as_bytes();
        result.is_negative = chars[0] == '-' as u8;

        let mut i = MAX_SIZE + result.is_negative as usize - chars.len();
        result.current_size = MAX_SIZE - i;

        while i < MAX_SIZE {
            result.digits[i] = chars[i + chars.len() - MAX_SIZE] - '0' as u8;
            i += 1;
        }  

        result
    }

    pub const fn from_i128(num: i128) -> Self {
        let mut result = Self::DEFAULT;
        result.is_negative = num < 0;
        let mut num = num.abs();
        let mut i = MAX_SIZE - 1;
        while num > 0 {
            result.digits[i] = (num % 10) as u8;
            num /= 10;
            i -= 1;
        }
        result.current_size = MAX_SIZE - i - 1;
        result
    }

    pub const fn const_clone(&self) -> Self {
        let mut result = Self::DEFAULT;
        result.is_negative = self.is_negative;
        result.current_size = self.current_size;
        let mut i = MAX_SIZE - self.current_size;
        while i < MAX_SIZE {
            result.digits[i] = self.digits[i];
            i += 1;
        }
        result
    }

    pub const fn max_size(&self) -> usize {
        MAX_SIZE
    }

    pub const fn size(&self) -> usize {
        self.current_size
    }

    pub const fn is_zero(&self) -> bool {
        self.current_size == 1 && self.digits[MAX_SIZE - 1] == 0
    }

    pub const fn is_even(&self) -> bool {
        self.digits[MAX_SIZE - 1] % 2 == 0
    }
    
    pub const fn add(self, other: Self) -> Self {
        let mut result: Self;

        // if signs are the same
        // we can simply add the numbers
        if !(self.is_negative ^ other.is_negative) { 
            result = Self::add_abs(self, other);
            result.is_negative = self.is_negative;
        } else {
            result = Self::sub_abs(self, other);
            result.is_negative = if self.abs_less(other) { other.is_negative } else { self.is_negative };
            if result.is_zero() {result.is_negative = false} 
        }
        result
    }

    #[inline(always)]
    const fn add_abs(self, other: Self) -> Self {
        let mut result: BigInt<MAX_SIZE> = Self {current_size: 0, ..Self::DEFAULT};
        let mut carry = 0;
        let mut i = MAX_SIZE - 1;
        while i > MAX_SIZE - const_helpers::max!(self.current_size, other.current_size) - 1 {
            let sum = self.digits[i] as i16 + other.digits[i] as i16 + carry;
            result.digits[i] = (sum % 10) as u8;
            carry = sum / 10;
            i -= 1;
        }
        if carry > 0 {
            result.digits[i] = carry as u8;
            result.current_size = 1;
        }
        result.current_size += const_helpers::max!(self.current_size, other.current_size);
        result
    }

    pub const fn sub(self, other: Self) -> Self {
        let mut result: Self;
  
        if !(self.is_negative ^ other.is_negative) {
            result = Self::sub_abs(self, other);
            result.is_negative = self.less(other);
        } else {
            result = Self::add_abs(self, other);
            result.is_negative = self.is_negative;
        }

        result
    }

    #[inline(always)]
    const fn sub_abs(self, other: Self) -> Self {
        let mut result = Self::DEFAULT;
        let greater: &Self;
        let smaller: &Self;

        if self.abs_less(other) {
            (greater, smaller) = (&other, &self);
        } else {
            (greater, smaller) = (&self, &other);
        }

        let mut borrow = 0;
        let mut i = MAX_SIZE - 1;
        let max_size = const_helpers::max!(self.current_size, other.current_size);
        let mut final_size = max_size;
        while i > MAX_SIZE - max_size - 1 {
            let diff = 10 + greater.digits[i] as i16 - smaller.digits[i] as i16 - borrow;
            (result.digits[i], borrow) = if diff >= 10 { ((diff - 10) as u8, 0i16) } else { (diff as u8, 1i16) };

            if result.digits[i] == 0 {
                final_size -= 1;
            } else {
                final_size = max_size;
            }
            i -= 1;
        }
        result.current_size = const_helpers::max!(final_size, 1);
        result
    }

    /// a.abs_less(b) returns true if |a| < |b|
    const fn abs_less(self, other: Self) -> bool {

        // if signs are the same, check the number lens
        if self.current_size != other.current_size {
            match self.current_size > other.current_size {
                // the sign is + and the number of digits is greater
                true => return false,
                // the sign is + and the number of digits is less
                false => return true,
            }
        }

        let size = self.current_size;

        let mut i = MAX_SIZE - size - 1;
        while i < MAX_SIZE {
            if self.digits[i] < other.digits[i] {
                return true;
            } else if self.digits[i] > other.digits[i] {
                return false;
            }
            i += 1;
        }
        false
    }

    /// a.less(b) returns true if a < b
    pub const fn less(self, other: Self) -> bool {

        // check if signs are different
        match (self.is_negative, other.is_negative) {
            (true, false) => return true,
            (false, true) => return false,
            _ => {}
        }

        // if signs are the same, check the number lens
        if self.current_size != other.current_size {
            match (self.current_size > other.current_size, self.is_negative & other.is_negative) {
                // the sign is + and the number of digits is greater
                (true, false) => return false,
                // the sign is + and the number of digits is less
                (false, false) => return true,
                // the sign is - and the number of digits is greater
                (true, true) => return true,
                // the sign is - and the number of digits is less
                (false, true) => return false,
            }
        }
           
        assert!(self.is_negative == other.is_negative, "signs are different");
        assert!(self.current_size == other.current_size, "sizes are different");

        let size = self.current_size;

        let mut i = MAX_SIZE - size - 1;
        while i < MAX_SIZE {
            if self.digits[i] < other.digits[i] {
                return if self.is_negative { false } else { true };
            } else if self.digits[i] > other.digits[i] {
                return if self.is_negative { true } else { false };
            }
            i += 1;
        }
        false
    }

    pub const fn equal(self, other: Self) -> bool {
        if self.is_negative != other.is_negative || self.current_size != other.current_size {
            return false;
        }

        let mut i = MAX_SIZE - self.current_size;
        while i < MAX_SIZE {
            if self.digits[i] != other.digits[i] {
                return false;
            }
            i += 1;
        }
        true
        
    }

    pub const fn greater(self, other: Self) -> bool {
        !self.less(other) && !self.equal(other)
    }


    pub const fn mul(self, other: Self) -> Self {
        // check if either of the numbers is zero
        if self.is_zero() || other.is_zero() {
            return Self::DEFAULT;
        }

        let mut result = Self::DEFAULT;
        let mut carry = 0;
        let mut i = MAX_SIZE - 1;
        while i > MAX_SIZE - other.current_size - 1 {
            let mut j = MAX_SIZE - 1;
            while j > MAX_SIZE - self.current_size - 1 {
                let loc = i + j - MAX_SIZE + 1;
                let mul = self.digits[j] as i16 * other.digits[i] as i16 + carry + result.digits[loc] as i16;
                result.digits[loc] = (mul % 10) as u8;
                carry = mul / 10;
                j -= 1;
            }
            result.digits[i + j - MAX_SIZE + 1] += carry as u8;
            carry = 0;
            i -= 1;
        }

        let current_size = self.current_size +  other.current_size;
        // check if the real length is less than current_size
        let mut i = MAX_SIZE - current_size;
        while i < MAX_SIZE {
            if result.digits[i] != 0 {
                result.current_size = MAX_SIZE - i;
                break;
            }
            i += 1;
        }
        result.is_negative = self.is_negative ^ other.is_negative;
        result
    }

    // self / other
    pub const fn div(self, other: Self) -> (Self, Self) {
        let mut quotient = Self::DEFAULT;
        let mut remainder = Self::DEFAULT;
        if self.is_zero() {
            return (quotient, remainder);
        }
        if other.is_zero() {
            panic!("division by zero");
        }


        let mut divident = self.const_clone();
        let len = other.current_size;
        let mut shift = 0;

        'outer: loop {
            let mut num = 0u8;
            'inner: loop {
                let lhs_pos = MAX_SIZE + shift - divident.current_size;
                let rhs_pos = MAX_SIZE - other.current_size;
                
                if lhs_pos + len - 1 >= MAX_SIZE && lhs_pos + len - 1 >= MAX_SIZE {
                    break 'outer;
                }

                // compare the two numbers
                let mut pos = 0;
                if MAX_SIZE + shift < divident.current_size + 1 || divident.digits[MAX_SIZE + shift - divident.current_size - 1] == 0 {
                    while pos < len {
                        if divident.digits[lhs_pos + pos] > other.digits[rhs_pos + pos] {
                            break; // divident is greater - subtract
                        } else if divident.digits[lhs_pos + pos] < other.digits[rhs_pos + pos] {
                            break 'inner; // divident is smaller - shift
                        }
                        pos += 1;
                    }
                }


                // subtract
                let mut pos = 0;
                let mut borrow = 0;
                let lhs_pos = MAX_SIZE - divident.current_size + shift + len - 1;
                let rhs_pos = MAX_SIZE - other.current_size + len - 1;
                while pos < len {
                    let diff = 10 + divident.digits[lhs_pos - pos] as i16 - other.digits[rhs_pos - pos] as i16 - borrow;
                    divident.digits[lhs_pos - pos] = (diff % 10) as u8;
                    borrow = (diff / 10) ^ 1;
                    pos += 1;
                }

                // subtract the borrow
                if lhs_pos >= pos {
                    assert!(divident.digits[lhs_pos - pos] as i16 >= borrow, "borrow is greater than the digit");
                    divident.digits[lhs_pos - pos] -= borrow as u8;
                } else {
                    assert!(borrow == 0, "borrow is not zero");
                }

                num += 1;
                
                
            }
            quotient.digits[shift] = num;
            shift += 1;

        }

        // move digits to the right
        let mut i = 1;
        while i <= shift {
            let value = quotient.digits[shift - i];
            quotient.digits[shift - i] = 0;
            quotient.digits[MAX_SIZE - i] = value;
            i += 1;
        }
        quotient.current_size = shift;

        // check if the real length is less than current_size
        let mut i = MAX_SIZE - shift;
        while i < MAX_SIZE {
            if quotient.digits[i] != 0 {
                quotient.current_size = MAX_SIZE - i;
                break;
            }
            i += 1;
        }

        // copy the remainder
        let mut i = MAX_SIZE - divident.current_size;
        while i < MAX_SIZE {
            remainder.digits[i] = divident.digits[i];
            i += 1;
        }

        // check if the real length is less than current_size
        let mut i = MAX_SIZE - divident.current_size;
        while i < MAX_SIZE {
            if remainder.digits[i] != 0 {
                remainder.current_size = MAX_SIZE - i;
                break;
            }
            i += 1;
        }

        quotient.is_negative = self.is_negative ^ other.is_negative;
        remainder.is_negative = self.is_negative | (self.is_negative && other.is_negative);

        quotient.current_size = const_helpers::max!(quotient.size(), 1);

        if quotient.is_zero() {
            quotient.is_negative = false;
        }

        if remainder.is_zero() {
            remainder.is_negative = false;
        }

        (quotient, remainder)
    }
    
}

pub mod const_helpers {
    #[macro_export]
    macro_rules! max {
        ($x:expr, $y:expr) => {
            if $x > $y { $x } else { $y }
        };
    }

    #[macro_export]
    macro_rules! min {
        ($x:expr, $y:expr) => {
            if $x < $y { $x } else { $y }
        };
    }

    pub(crate) use max;
    #[allow(unused_imports)]
    pub(crate) use min;
}



// TESTS
mod tests {
    use crate::bigint::BigInt;
    #[allow(dead_code)]
    type BigIntTest = BigInt<100>;

    #[test]
    fn from_positive() {
        const STR: &str = "123456789123456789123456789123456789";
        const X: BigIntTest = BigInt::from_str(STR);
        const DIGITS: [u8; X.max_size()] = { 
            let mut result =  [0u8; X.max_size()]; 

            let digits = BigIntTest::DEFAULT.max_size() - STR.len();
            let mut i = digits;
            while i < BigIntTest::DEFAULT.max_size() {
                result[i as usize] = ((i - digits) % 9 + 1) as u8;
                i+=1;
            }

            result
        };

        const EXPECTED: BigIntTest = BigInt { is_negative: false, digits:DIGITS, current_size: STR.len()}; 
        assert_eq!(X, EXPECTED);
    }

    #[test]
    fn from_negative() {
        const STR: &str = "-123456789123456789123456789123456789";
        const X: BigIntTest = BigInt::from_str(STR);
        const DIGITS: [u8; X.max_size()] = { 
            let mut result =  [0u8; X.max_size()]; 

            let digits = BigIntTest::DEFAULT.max_size() - STR.len() + 1;
            let mut i = digits;
            while i < BigIntTest::DEFAULT.max_size() {
                result[i as usize] = ((i - digits) % 9 + 1) as u8;
                i+=1;
            }

            result
        };

        const EXPECTED: BigIntTest = BigInt { is_negative: true, digits:DIGITS, current_size: STR.len() - 1};
        assert_eq!(X, EXPECTED);
    }

    #[test]
    fn from_i128() {
        {
            const NUM: i128 = 1234567891234567891;
            const X: BigIntTest = BigInt::from_i128(NUM);
            const Y: BigIntTest = BigInt::from_str("1234567891234567891");
            assert_eq!(X, Y);
        }

        {
            const NUM: i128 = -1234567891234567891;
            const X: BigIntTest = BigInt::from_i128(NUM);
            const Y: BigIntTest = BigInt::from_str("-1234567891234567891");
            assert_eq!(X, Y);
        }
    }

    #[test]
    fn less() {
        for i in -1000..1000 {
            for j in -1000..1000 {
                let i_str = i.to_string();
                let j_str = j.to_string();
                let i1: BigIntTest = BigInt::from_str(&i_str);
                let j1: BigIntTest = BigInt::from_str(&j_str);
                let result = i < j;
                let expected = i1 < j1; 
                assert_eq!(result, expected);
            }
        }
    }

    #[test]
    fn greater() {
        for i in -1000..1000 {
            for j in -1000..1000 {
                let i_str = i.to_string();
                let j_str = j.to_string();
                let i1: BigIntTest = BigInt::from_str(&i_str);
                let j1: BigIntTest = BigInt::from_str(&j_str);
                let result = i > j;
                let expected = i1 > j1;
                assert_eq!(result, expected);
            }
        }
    }

    #[test]
    fn equal() {
        for i in -1000..1000 {
            for j in -1000..1000 {
                let i_str = i.to_string();
                let j_str = j.to_string();
                let i1: BigIntTest = BigInt::from_str(&i_str);
                let j1: BigIntTest = BigInt::from_str(&j_str);
                let result = i == j;
                let expected = i1 == j1;
                assert_eq!(result, expected);
            }
        }
    }

    #[test]
    fn add(){
        {
            const STR1: &str = "-1";
            const STR2: &str = "999";
            const RESULT: &str = "998";
            const X: BigIntTest = BigInt::from_str(STR1);
            const Y: BigIntTest = BigInt::from_str(STR2);
            
            const Z: BigIntTest = X.add(Y);

            const EXPECTED: BigIntTest = BigInt::from_str(RESULT);
            assert_eq!(Z, EXPECTED);
        }
        {
            const STR1: &str = "-123456789123456789123456789123456789";
            const STR2: &str = "-987654321987654321987654321987654321";
            const RESULT: &str = "-1111111111111111111111111111111111110";
            const X: BigIntTest = BigInt::from_str(STR1);
            const Y: BigIntTest = BigInt::from_str(STR2);

            const Z: BigIntTest = X.add(Y);

            const EXPECTED: BigIntTest = BigInt::from_str(RESULT);
            assert_eq!(Z, EXPECTED);
        }
    }

    #[test]
    fn add_2() {
        // positive + positive
        for x in -1000..=1000 {
            for y in -1000..=1000 {

                let x_str = x.to_string();
                let y_str = y.to_string();
                let result = (x + y).to_string();
                let x1: BigIntTest = BigInt::from_str(&x_str);
                let y1: BigIntTest = BigInt::from_str(&y_str);
                let result1: BigIntTest = x1 + y1;
                let expected: BigIntTest = BigInt::from_str(&result);
                assert_eq!(result1, expected, "{}", format!("{} + {} = {}", x, y, result));
            }
        }
    }

    #[test]
    fn sub(){
        {
            const STR2: &str = "987654321987654321987654321987654321";
            const STR1: &str = "123456789123456789123456789123456789";

            const RESULT: &str = "864197532864197532864197532864197532";

            const X: BigIntTest = BigInt::from_str(STR1);
            const Y: BigIntTest = BigInt::from_str(STR2);
            const Z: BigIntTest = Y.sub(X);

            const EXPECTED: BigIntTest = BigInt::from_str(RESULT);
            assert_eq!(Z, EXPECTED);
        }
        {
            const STR2: &str = "-987654321987654321987654321987654321";
            const STR1: &str = "-123456789123456789123456789123456789";

            const RESULT: &str = "-864197532864197532864197532864197532";

            const X: BigIntTest = BigInt::from_str(STR1);
            const Y: BigIntTest = BigInt::from_str(STR2);
            const Z: BigIntTest = Y.sub(X);

            const EXPECTED: BigIntTest = BigInt::from_str(RESULT);
            assert_eq!(Z, EXPECTED);
        }


    }

    #[test]
    fn sub_2() {
        for x in -1000..=1000i32 {
            for y in -1000..=1000i32 {
                let x_str = x.to_string();
                let y_str = y.to_string();
                let result = (x - y).to_string();
                let x1: BigIntTest = BigInt::from_str(&x_str);
                let y1: BigIntTest = BigInt::from_str(&y_str);
                let result1: BigIntTest = x1 - y1;
                let expected: BigIntTest = BigInt::from_str(&result);
                assert_eq!(result1, expected, "{}", format!("{} - {} = {}", x, y, result));
            }
        }
    }

    #[test]
    fn mul(){
        for x in -1000..=1000i32 {
            for y in -1000..=1000i32 {
                let x_str = x.to_string();
                let y_str = y.to_string();
                let result = (x * y).to_string();
                let x1: BigIntTest = BigInt::from_str(&x_str);
                let y1: BigIntTest = BigInt::from_str(&y_str);
                let result1: BigIntTest = x1 * y1;
                let expected: BigIntTest = BigInt::from_str(&result);
                assert_eq!(result1, expected, "{}", format!("{} * {} = {}", x, y, result));
            }
        }
    }

    #[test]
    fn div(){
        type I3 = BigInt<3>;
        for x in -999..=999i32 {
            for y in -999..=999i32 {
                if y == 0 {
                    continue;
                }
                let x_str = x.to_string();
                let y_str = y.to_string();
                let result = (x / y).to_string();
                let result_rem = (x % y).to_string();
                let x1: I3 = BigInt::from_str(&x_str);
                let y1: I3 = BigInt::from_str(&y_str);
                let (result1, result2) = x1.div(y1);
                let expected: I3 = BigInt::from_str(&result);
                let expected_rem: I3 = BigInt::from_str(&result_rem);
                assert_eq!(result1, expected, "{}", format!("{} / {} = {}", x, y, result));
                assert_eq!(result2, expected_rem, "{}", format!("{} % {} = {}", x, y, result_rem));
            }
        }
    }

    #[test]
    fn div_2() {
        const A: BigIntTest = BigInt::from_str("33322211112345678987654321");
        const B: BigIntTest = BigInt::from_str("15485863");
        const DIV: BigIntTest = BigInt::from_str("2151782636353277759");
        const REM: BigIntTest = BigInt::from_str("10833304");

        const RES: (BigIntTest, BigIntTest) = A.div(B);
        assert_eq!(RES.0, DIV);
        assert_eq!(RES.1, REM);
    }
}
