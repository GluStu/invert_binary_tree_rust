use std::fmt::Display;

#[derive(Debug, Default, Clone)]
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

fn generate_tree(level: usize, counter: &mut i32) -> Option<Box<TreeNode<i32>>> {
    if level == 0{
        return None;
    }
    else{
        let mut node = TreeNode{val: *counter, left: None, right: None};
        node.val = *counter;
        *counter += 1;
        node.left = generate_tree(level - 1, counter);
        node.right = generate_tree(level - 1, counter);
        Some(Box::new(node.clone()))
    }
}

fn print_tree<T: Display>(root: &Option<Box<TreeNode<T>>>, level: usize) {
    match root{
        Some(node) => {
            print_tree(&node.left, level + 1);
            for _ in 0..level{
                print!("   ")
            }
            println!("{}", &node.val);
            print_tree(&node.right, level + 1);
        },
        None => {},
    }
}

fn invert_tree<T>(root: Option<Box<TreeNode<T>>>) -> Option<Box<TreeNode<T>>> {
    // match root {
    //     Some(mut node) => {
    //         let left = invert_tree(node.right.take());
    //         let right = invert_tree(node.left.take());
    //         node.left = left;
    //         node.right = right;
    //         Some(node)
    //     }
    //     None => None,
    // }

    if let Some(mut node) = root {
        let left = invert_tree(node.right.take());
        let right = invert_tree(node.left.take());
        node.left = left;
        node.right = right;
        return Some(node);
    }
    None
}
fn main() {
    let mut counter = 1;
    let tree = generate_tree(3, &mut counter);
    print_tree(&tree, 3);
    println!("-----------------------");
    print_tree(&invert_tree(tree),3);
}

// use std::rc::Rc;
// use std::cell::RefCell;
// impl Solution {
//     pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
//         if root.is_none(){
//             return None;
//         }
//         if let Some(node) = root.clone(){
//             let mut node_borr = node.borrow_mut();

//             let temp = node_borr.left.clone();
//             node_borr.left = Self::invert_tree(node_borr.right.clone());
//             node_borr.right = Self::invert_tree(temp.clone());
//         }
//         root
//     }
// }