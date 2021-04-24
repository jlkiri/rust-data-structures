pub struct Node<'a, T> {
  value: T,
  edges: Vec<&'a Node<'a, T>>
}

impl<'a, T: PartialEq> Node<'a, T> {
  pub fn new(value: T) -> Self {
    Self {
      value,
      edges: Vec::new()
    }
  }
}

pub struct Graph<'a, T> {
  nodes: Vec<Node<'a, T>>
}

impl<'a, T: PartialEq> Graph<'a, T> {
  pub fn new() -> Self {
    Self {
      nodes: Vec::new()
    }
  }

  pub fn add_node(&mut self, value: T) {
    self.nodes.push(Node::new(value))
  }
  
  pub fn get_node(&self, value: T) -> Option<&Node<T>> {
    self.nodes.iter().find(|node| node.value == value)
  }

  pub fn add_edge(&mut self, value_a: T, value_b: T) {
    let ref mut node_a = self.get_node(value_a);
    let mut node_b = self.get_node(value_b);
    match (node_a, node_b) {
      (Some(a), Some(b)) => {
        a.edges.push(b);
      },
      _ => panic!("Node not found!")
    }
  }
}