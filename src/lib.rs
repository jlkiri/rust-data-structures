mod linked_list;
mod queue;
mod stack;
mod graph;

use linked_list::*;
use queue::*;
use stack::*;
use graph::*;

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

  #[test]
  fn linked_push() {
    let mut list: List<isize> = List::new();
    list.push(1);
    list.push(2);
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));
  }

  #[test]
  fn linked_peek() {
    let mut list: List<isize> = List::new();
    list.push(1);
    list.push(2);
    assert_eq!(list.peek(), Some(&2));
  }
}
