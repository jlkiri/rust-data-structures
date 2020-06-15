struct Stack<T> {
  stack: Vec<T>,
}

impl<T> Stack<T> {
  fn new() -> Stack<T> {
    Stack { stack: Vec::new() }
  }

  fn length(&self) -> usize {
    self.stack.len()
  }

  fn pop(&mut self) -> Option<T> {
    self.stack.pop()
  }

  fn push(&mut self, item: T) {
    self.stack.push(item)
  }

  fn peek(&self) -> Option<&T> {
    self.stack.get(self.length() - 1)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn stack_peek() {
    let mut stack: Stack<isize> = Stack::new();
    stack.push(1);
    assert_eq!(*stack.peek().unwrap(), 1);
  }

  #[test]
  #[should_panic]
  fn stack_panic() {
    let mut stack: Stack<isize> = Stack::new();
    let last_element = stack.peek().unwrap();
  }
}
