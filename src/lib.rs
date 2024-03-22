#[derive(Debug)]
struct Tree<T> {
	root: Option<Box<Node<T>>>
}

#[derive(Debug)]
struct Node<T> {
	left: Option<Box<Node<T>>>,
	right: Option<Box<Node<T>>>,
	value: T
}

impl<T: Ord> Node<T> {
	fn new(data: T) -> Self {
		Node {
			left: None,
			right: None, 
			value: data
		}
	}

	fn compare(left: &T, right: &T) -> bool {
		left < right
	}
}

impl<T> From<Node<T>> for Option<Box<Node<T>>> {
	fn from(node: Node<T>) -> Self {
		Some(Box::new(node))
	}
}

impl<T: Ord> Tree<T> {
	fn new() -> Self {
		Tree { root: None }
	}

	fn insert(&mut self, value: T) {
		match &mut self.root {
			None => self.root = Node::new(value).into(),
			Some(node) => {
				let mut root = node;
				loop {
					if root.left.is_none() {
						root.left = Node::new(value).into();
						return 
					}

					if root.right.is_none() {
						root.right = Node::new(value).into();
						return
					}

					let left = root.left.as_mut().unwrap();
					let right = root.right.as_mut().unwrap();
			
					if left.value > value {
						root = left;
						continue;
					}

					root = right;
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn tree_creation_should_work() {
		let mut tree: Tree::<i32> = Tree::new();

		tree.insert(420);
		tree.insert(69);
		tree.insert(100);
		tree.insert(1337);

		assert!(tree.root.is_some());
	}

	#[test]
	pub fn tree_when_created_should_have_correct_nodes() {
		let mut tree: Tree::<i32> = Tree::new();

		tree.insert(420);
		tree.insert(69);

		assert_eq!(tree.root.unwrap().left.unwrap().value, 69)
	}
}
