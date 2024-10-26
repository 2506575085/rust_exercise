pub mod my_default {
  #[derive(Debug)]
  pub struct TestDefault {
    pub x: i32,
    pub y: i32,
  }
  impl MyDefault for TestDefault {
    fn default() -> Self {
      TestDefault { x: 0, y: 0 }
    }
  }
  pub trait MyDefault {
    fn default() -> Self;
  }
  // 在TestDefault结构体实例化中调用时,编译器将T单态化为TestDefault,等效于调用TestDefault::default
  pub fn default<T:MyDefault>() -> T {
    T::default()
  }
}


pub mod test_match {
  use super::my_default::TestDefault;
  pub fn test_match() {
    let item = TestDefault { x: 1, y: 2 };
    match item {
      // 匹配结构体单个字段且获取字段值需使用@操作符
      TestDefault { x: x @ 1..2, .. } => {
        println!("x is {}", x)
      }
      _ => {}
    }
  }
}


mod test_trait {
  use std::fmt;

  trait Iterator {
    type Item;
    fn next(&self) -> Option<Self::Item>;
  }
  trait IteratorType<T> {
    fn next(&self) -> Option<T>;
  }
  #[derive(Debug)]
  struct TestIterator;
  impl Iterator for TestIterator {
    type Item = TestIterator;
    fn next(&self) -> Option<Self::Item> {
      Some(TestIterator {})
    }
  }
  // 指定关联类型的trait只能被实现一次，以下代码无法被编译
  // impl Iterator for TestIterator {
  //     type Item = TestIteratorType;
  //     fn next(self) -> Option<Self::Item> {
  //         Some(TestIteratorType {})
  //     }
  // }

  struct TestIteratorType;
  // 泛型trait可被多次实现，但多次实现后每次方法调用必须指定类型
  impl IteratorType<TestIteratorType> for TestIteratorType {
    fn next(&self) -> Option<TestIteratorType> {
      Some(TestIteratorType {})
    }
  }
  impl IteratorType<TestIterator> for TestIteratorType {
    fn next(&self) -> Option<TestIterator> {
      Some(TestIterator {})
    }
  }

  fn test_iterator() {
    let iter = TestIterator {};
    iter.next().unwrap().next().unwrap().next();
    let iter_type = TestIteratorType {};
    <TestIteratorType as IteratorType<TestIteratorType>>::next(&iter_type);
    // iter_type.next().unwrap().next(); // 无法编译，需指定类型
  }

  trait OutlinePrint: fmt::Display {
      fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
      }
  }
  impl fmt::Display for TestIterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
  }
  impl OutlinePrint for TestIterator {}

  // 为mod外部类型实现trait
  struct Wrapper(Vec<String>);
  impl fmt::Display for Wrapper {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "[{}]", self.0.join(", "))
      }
  }
}
