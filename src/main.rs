// Import Rc (Reference Counting) and RefCell for interior mutability
use std::rc::Rc;
use std::cell::RefCell;
use std::io;


// Define the TreeNode struct with Rc and RefCell
#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    // Constructor function for TreeNode
    fn new(value: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
        }))
    }
}

// Insert function for inserting a value into the BST
fn insert(root: Option<Rc<RefCell<TreeNode>>>, value: i32) -> Option<Rc<RefCell<TreeNode>>> {
    // If root is None, create a new TreeNode and return its Rc
    if root.is_none() {
        return Some(TreeNode::new(value));
    }

    // Unwrap Rc and RefCell to get mutable access to TreeNode
    let root = root.unwrap();

    // Compare value with current node's value
    if value < root.borrow().value {
        // Insert into the left subtree recursively
        let new_left = insert(root.borrow().left.clone(), value);
        root.borrow_mut().left = new_left;
    } else {
        // Insert into the right subtree recursively
        let new_right = insert(root.borrow().right.clone(), value);
        root.borrow_mut().right = new_right;
    }

    // Return the modified root
    Some(root)
}

// Function to print the tree in an in-order traversal
fn print_tree(root: Option<Rc<RefCell<TreeNode>>>, indent: usize) {
    if let Some(node) = root {
        // Print right subtree with increased indentation
        print_tree(node.borrow().right.clone(), indent + 4);

        // Print the current node's value with indentation
        println!("{:indent$}{}", "", node.borrow().value, indent = indent);

        // Print left subtree with increased indentation
        print_tree(node.borrow().left.clone(), indent + 4);
    }
}

fn main() {
    // Create an empty tree (None)
    let mut root = None;

    let mut input_line = String::new();

    println!("enter numbers separated by space i.e 10 5 15 3 7 12 18 :");

    io::stdin().read_line(&mut input_line)
        .expect("Failed to read line");

    let values: Vec<i32> = input_line.split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();

    for &value in &values {
        root = insert(root, value);
    }

    // Print the tree
    print_tree(root, 0);
}
