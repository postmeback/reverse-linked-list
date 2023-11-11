use std::fmt::Debug;

// Define a basic Node structure for the linked list
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// Define the LinkedList structure
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Debug> LinkedList<T> {
    // Create a new empty linked list
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Insert a value at the beginning of the linked list
    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Reverse the linked list
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut node) = current.take() {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            current = next;
        }

        self.head = prev;
    }

    // Print the linked list
    pub fn print(&self) {
        let mut current = &self.head;

        print!("Linked List: ");

        while let Some(node) = current {
            print!("{:?} -> ", node.value);
            current = &node.next;
        }

        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();

    // Push values to the linked list
    list.push(1);
    list.push(2);
    list.push(3);

    // Print the original linked list
    println!("Original Linked List:");
    list.print();

    // Reverse the linked list
    list.reverse();

    // Print the reversed linked list
    println!("Reversed Linked List:");
    list.print();
}
