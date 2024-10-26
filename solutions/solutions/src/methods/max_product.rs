use std::{cell::RefCell, rc::Rc};
use super::Solution;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

pub enum NodeType {
    Left,
    Right,
    None
}
// 为避免递归重复传递外部可变引用，单独构造一个类存储外部可变引用
pub struct MaxProductDfs<'a> {
    vector: &'a mut Vec<i128>,
    total: &'a mut i128,
}
impl MaxProductDfs<'_> {
    pub fn new<'a>(vector: &'a mut Vec<i128>, total: &'a mut i128) -> MaxProductDfs<'a> {
        MaxProductDfs {
            vector,
            total
        }
    }
    pub fn run(&mut self, node: &Rc<RefCell<TreeNode>>) {
        self.dfs(node, &mut (0,0), NodeType::None);
    }
    fn dfs(&mut self, node: &Rc<RefCell<TreeNode>> ,parant_count: &mut (i128, i128), node_type: NodeType) {
        let left_clone = node.as_ref().borrow().left.clone();
        let right_clone = node.as_ref().borrow().right.clone();
        let mut self_count = (0, 0);
        if let Some(left) = &left_clone {
            self.dfs(left, &mut self_count, NodeType::Left);
        }
        if let Some(right) = &right_clone {
            self.dfs(right, &mut self_count, NodeType::Right);
        }
        if self_count.0 != 0 { self.vector.push(self_count.0); }
        if self_count.1 != 0 { self.vector.push(self_count.1); }

        let count = self_count.0 + self_count.1 + node.as_ref().borrow().val as i128;
        match node_type {
            NodeType::Left => {
                parant_count.0 += count;
            },
            NodeType::Right => {
                parant_count.1 += count;
            },
            NodeType::None => {
                *self.total += count;
            }
        }
    }
}
impl Solution {
    // 1339
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // let mut count = (0, 0);
        let mut res_count_vec = vec![];
        let mut total = 0;

        // fn dfs(node: &Rc<RefCell<TreeNode>> ,parant_count: &mut (i128, i128), node_type: NodeType, vec: &mut Vec<i128>, total: &mut i128) {
        //     let left_clone = node.as_ref().borrow().left.clone();
        //     let right_clone = node.as_ref().borrow().right.clone();
        //     let mut self_count = (0, 0);
        //     if let Some(left) = &left_clone {
        //         dfs(left, &mut self_count, NodeType::Left, vec, total);
        //     }
        //     if let Some(right) = &right_clone {
        //         dfs(right, &mut self_count, NodeType::Right, vec, total);
        //     }
        //     if self_count.0 != 0 { vec.push(self_count.0); }
        //     if self_count.1 != 0 { vec.push(self_count.1); }
        //     let count = self_count.0 + self_count.1 + node.as_ref().borrow().val as i128;
        //     match node_type {
        //         NodeType::Left => {
        //             parant_count.0 += count;
        //         },
        //         NodeType::Right => {
        //             parant_count.1 += count;
        //         },
        //         NodeType::None => {
        //             *total += count;
        //         }
        //     }
        // }
        // dfs(&root.unwrap(), &mut count, NodeType::None, &mut res_count_vec, &mut total);

        let mut dfs_constuctor = MaxProductDfs::new(&mut res_count_vec, &mut total);
        dfs_constuctor.run(&root.unwrap());

        let mut max_half = 0;
        res_count_vec.iter().for_each(|x| {
            if (*x * 2 - total).abs() <= (max_half * 2 - total).abs() {
                max_half = *x;
            }
        });
        ((total - max_half).abs() * max_half % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let mut tree = TreeNode::new(1);
    tree.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    tree.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    tree.right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    tree.left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    tree.left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    println!("{:?}",tree);
    println!("{:?}",Solution::max_product(Some(Rc::new(RefCell::new(tree)))));
  }
}