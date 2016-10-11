use std::rc::Rc;

pub trait Expression {
  fn evaluate(&self) -> f64;
}

#[derive(Debug, Clone, Copy)]
struct Value {
  val: i32,
}

impl Value {
  fn new(val: i32) -> Value {
    Value {
      val: val,
    }
  }
}

impl Expression for Value {
  fn evaluate(&self) -> f64 {
    self.val as f64
  }
}

#[derive(Debug, Clone, Copy)]
enum UnaryOp {
  Neg,
}

#[derive(Clone)]
struct UnaryExpr {
  op: UnaryOp,
  value: Rc<Expression>,
}

impl UnaryExpr {
  fn new(op: UnaryOp, value: Rc<Expression>) -> UnaryExpr {
    UnaryExpr {
      op: op,
      value: value,
    }
  }
}

impl Expression for UnaryExpr {
  fn evaluate(&self) -> f64 {
    match self.op {
      UnaryOp::Neg => -self.value.evaluate()
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum BinOp {
  Add,
  Sub,
  Mul,
  Div,
}

#[derive(Clone)]
struct BinaryExpr {
  op: BinOp,
  left: Rc<Expression>,
  right: Rc<Expression>,
}

impl BinaryExpr {
  fn new(op: BinOp, left: Rc<Expression>, right: Rc<Expression>) -> BinaryExpr {
    BinaryExpr {
      op: op,
      left: left,
      right: right,
    }
  }
}

impl Expression for BinaryExpr {
  fn evaluate(&self) -> f64 {
    let left = self.left.evaluate();
    let right = self.right.evaluate();
    match self.op {
      BinOp::Add => left + right,
      BinOp::Sub => left - right,
      BinOp::Mul => left * right,
      BinOp::Div => left / right,
    }
  }
}

#[test]
fn test_value() {
  assert_eq!(1f64, Value::new(1).evaluate());
}

#[test]
fn test_unary() {
  assert_eq!(-1f64, UnaryExpr::new(UnaryOp::Neg, Rc::new(Value::new(1))).evaluate());
}

#[test]
fn test_binary() {
  assert_eq!(5.0, BinaryExpr::new(BinOp::Add, Rc::new(Value::new(1)), Rc::new(Value::new(4))).evaluate());
  assert_eq!(-3.0, BinaryExpr::new(BinOp::Sub, Rc::new(Value::new(1)), Rc::new(Value::new(4))).evaluate());
  assert_eq!(4.0, BinaryExpr::new(BinOp::Mul, Rc::new(Value::new(1)), Rc::new(Value::new(4))).evaluate());
  assert_eq!(0.25, BinaryExpr::new(BinOp::Div, Rc::new(Value::new(1)), Rc::new(Value::new(4))).evaluate());
}
