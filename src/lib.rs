mod queue;
mod stack;

use queue::*;
use stack::*;

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
  fn stack_pop() {
    let mut stack: Stack<isize> = Stack::new();
    stack.push(1);
    let item = stack.pop();
    assert_eq!(item.unwrap(), 1);
    assert_eq!(stack.is_empty(), true);
  }

  #[test]
  #[should_panic]
  fn stack_empty_peek_panic() {
    let stack: Stack<isize> = Stack::new();
    stack.peek().unwrap();
  }

  #[test]
  fn queue_peek() {
    let mut queue: Queue<isize> = Queue::new();
    queue.enqueue(1);
    assert_eq!(*queue.peek().unwrap(), 1);
  }

  #[test]
  fn queue_dequeue() {
    let mut queue: Queue<isize> = Queue::new();
    queue.enqueue(1);
    let item = queue.dequeue();
    assert_eq!(item, 1);
    assert_eq!(queue.is_empty(), true);
  }
}
