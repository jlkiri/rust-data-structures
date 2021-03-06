use std::mem;

struct Node<T> {
  value: T,
  next: Option<Box<Node<T>>>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
  head: Link<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { head: None }
  }

  pub fn push(&mut self, item: T) {
    let next_node = Box::new(Node {
      value: item,
      next: self.head.take(),
    });

    self.head = Some(next_node)
  }

  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|node| {
      self.head = node.next;
      node.value
    })
  }

  pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.value)
  }
}
