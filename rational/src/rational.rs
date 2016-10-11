use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::string::ToString;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rational {
  n: i32,
  d: i32, 
}

impl Rational {
  fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
			a
    }
    else {
      Rational::gcd(b, a % b)
    }
  }
  
  pub fn new(n: i32, d: i32) -> Rational {
    let n = if d < 0 { -n } else { n };
    let d = d.abs();
    let g = Rational::gcd(n.abs(), d);
    Rational {
      n: n / g,
      d: d / g,
    }
  }
}

impl ToString for Rational {
  fn to_string(&self) -> String {
    self.n.to_string() + "/" + self.d.to_string().as_str()
  }
}

impl Add for Rational {
  type Output = Rational;
  fn add(self, rhs: Rational) -> Self::Output {
    Rational::new(self.n * rhs.d + self.d * rhs.n, self.d * rhs.d)
    
  }
}

impl Add<i32> for Rational {
  type Output = Rational;
  fn add(self, rhs: i32) -> Self::Output {
    Rational::new(self.d * rhs + self.n, self.d)
  }
}

impl Add<Rational> for i32 {
  type Output = Rational;
  fn add(self, rhs: Rational) -> Self::Output {
    rhs + self
  }
}

impl Sub for Rational {
  type Output = Rational;
  fn sub(self, rhs: Rational) -> Self::Output {
    Rational::new(self.n * rhs.d - self.d * rhs.n, self.d * rhs.d)
    
  }
}

impl Sub<i32> for Rational {
  type Output = Rational;
  fn sub(self, rhs: i32) -> Self::Output {
    Rational::new(self.n - self.d * rhs, self.d)
  }
}

impl Sub<Rational> for i32 {
  type Output = Rational;
  fn sub(self, rhs: Rational) -> Self::Output {
    Rational::new(rhs.d * self - rhs.n, rhs.d)
  }
}

impl Mul for Rational {
  type Output = Rational;
  fn mul(self, rhs: Rational) -> Self::Output {
    Rational::new(self.n * rhs.n, self.d * rhs.d)
  }
}

impl Mul<i32> for Rational {
  type Output = Rational;
  fn mul(self, rhs: i32) -> Self::Output {
    Rational::new(self.n * rhs, self.d)
  }
}

impl Mul<Rational> for i32 {
  type Output = Rational;
  fn mul(self, rhs: Rational) -> Self::Output {
    rhs * self
  }
}

impl Neg for Rational {
  type Output = Rational;
  fn neg(self) -> Self::Output {
    Rational::new(-self.n, self.d)
  }
}

#[test]
fn new_test() {
  assert_eq!("1/3", Rational::new(1, 3).to_string());
  assert_eq!("1/3", Rational::new(6, 18).to_string());
  assert_eq!("1/3", Rational::new(-6, -18).to_string());
  assert_eq!("-1/3", Rational::new(6, -18).to_string());
  assert_eq!("-1/3", Rational::new(-6, 18).to_string());
}

#[test]
fn add_test() {
  let instance1 = Rational::new(1, 3);
  let instance2 = Rational::new(3, 2);
  let instance3 = Rational::new(4, 9);
  let res1 = instance1 + 1;
  let res2 = 1 + instance1;
  let res3 = instance1 + instance2;
  let res4 = instance1 + instance3;
  assert_eq!("4/3", res1.to_string());
  assert_eq!("4/3", res2.to_string());
  assert_eq!("11/6", res3.to_string());
  assert_eq!("7/9", res4.to_string());
}

#[test]
fn sub_test() {
  let instance1 = Rational::new(1, 3);
  let instance2 = Rational::new(3, 2);
  let instance3 = Rational::new(4, 9);
  let res1 = instance1 - 1;
  let res2 = 1 - instance1;
  let res3 = instance1 - instance2;
  let res4 = instance1 - instance3;
  assert_eq!("-2/3", res1.to_string());
  assert_eq!("2/3", res2.to_string());
  assert_eq!("-7/6", res3.to_string());
  assert_eq!("-1/9", res4.to_string());
}

#[test]
fn mul_test() {
  let instance1 = Rational::new(1, 3);
  let instance2 = Rational::new(3, 2);
  let instance3 = Rational::new(4, 9);
  let res1 = instance1 * 2;
  let res2 = 2 * instance1;
  let res3 = instance1 * instance2;
  let res4 = instance1 * instance3;
  assert_eq!("2/3", res1.to_string());
  assert_eq!("2/3", res2.to_string());
  assert_eq!("1/2", res3.to_string());
  assert_eq!("4/27", res4.to_string());
}

#[test]
fn neg_test() {
  let instance1 = Rational::new(1, 3);
  let instance2 = Rational::new(1, -3);
  let res1 = -instance1;
  let res2 = -instance2;
  assert_eq!("-1/3", res1.to_string());
  assert_eq!("1/3", res2.to_string());
}
