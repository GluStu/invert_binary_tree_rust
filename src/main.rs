use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
  pub val: T,
  pub left: Option<Box<TreeNode<T>>>,
  pub right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
  #[inline]
  pub fn new(val: T) -> Self {
    TreeNode { val: val, left: None, right: None}
  }

  pub fn insert_left(&mut self, val: T){
        self.left = Some(Box::new(TreeNode::new(val)))
  }

  pub fn insert_right(&mut self, val: T){
    self.right = Some(Box::new(TreeNode::new(val)))
  }
}

#[derive(Debug)]
enum Action<T, U> {
    Call(T),
    Handle(U)
}

fn generate_tree(level: usize) -> Option<Box<TreeNode<i32>>> {
    let mut counter = 1;
    let mut arg_stack = Vec::<Action<usize, i32>>::new();
    let mut ret_stack: Vec<Option<Box<TreeNode<i32>>>> = Vec::new();

    use Action::*;
    arg_stack.push(Call(level));
    while let Some(action) = arg_stack.pop() {
        match action {
            Call(level) => if level > 0 {
                arg_stack.push(Handle(counter));
                counter += 1;
                arg_stack.push(Call(level - 1));
                arg_stack.push(Call(level - 1));
            } else {
                ret_stack.push(None);
            },
            Handle(val) => {
                let left = ret_stack.pop().unwrap();
                let right = ret_stack.pop().unwrap();
                ret_stack.push(Some(Box::new(TreeNode{val, left, right})));
            },
        }
    }

    ret_stack.pop().unwrap()
}

fn print_tree<T: Display>(root: &Option<Box<TreeNode<T>>>) {
    let mut stack = Vec::<Action<(&Option<Box<TreeNode<T>>>, usize), (&T, usize)>>::new();
    use Action::*;
    stack.push(Call((root, 0)));
    while let Some(action) = stack.pop() {
        match action {
            Call((root, level)) => if let Some(node) = root {
                stack.push(Call((&node.left, level + 1)));
                stack.push(Handle((&node.val, level)));
                stack.push(Call((&node.right, level + 1)));
            },
            Handle((value, level)) => {
                for _ in 0..level {
                    print!("  ")
                }
                println!("{}", value);
            }
        }
    }
}

fn invert_tree<T>(root: Option<Box<TreeNode<T>>>) -> Option<Box<TreeNode<T>>> {
    match root {
        Some(mut node) => {
            let left = invert_tree(node.right.take());
            let right = invert_tree(node.left.take());
            node.left = left;
            node.right = right;
            Some(node)
        }
        None => None,
    }
}
fn main() {
    let tree = generate_tree(3);
    print_tree(&tree);
    println!("-----------------------");
    print_tree(&invert_tree(tree));
}