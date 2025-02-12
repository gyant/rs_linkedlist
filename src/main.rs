#[derive(Debug, PartialEq)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug, PartialEq)]
struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    fn insert_at_beginning(&mut self, value: i32) {
        let mut new_node = Box::new(Node {
            data: value,
            next: None,
        });

        if !self.head.is_none() {
            new_node.next = self.head.take();
        }

        self.head = Some(new_node);
        self.size += 1;
    }

    fn insert_at_end(&mut self, value: i32) {
        let new_node = Box::new(Node {
            data: value,
            next: None,
        });

        if self.head.is_none() {
            self.head = Some(new_node);
            self.size += 1;
            return;
        }

        let mut current = self.head.as_mut();

        while let Some(node) = current {
            if node.next.is_none() {
                node.next = Some(new_node);
                self.size += 1;
                break;
            }

            current = node.next.as_mut();
        }
    }

    fn delete(&mut self, value: i32) -> Option<i32> {
        // Handle first case
        if let Some(node) = &self.head {
            let old_head = self.head.take()?;
            self.head = old_head.next;
            return Some(old_head.data);
        }

        let mut previous = self.head.as_mut();
        let mut current = None;

        // Handle when single length list and item not found.
        if let Some(node) = previous.as_mut() {
            if node.next.is_none() {
                return None;
            }

            current = node.next.as_mut();
        }

        println!("previous: {:?}", previous);
        println!("current: {:?}", current);

        //while let Some(node) = current {
        //    if node.data == value {
        //        let node_to_delete = node.as_mut();

        //        if let Some(previous_node) = previous {
        //            previous_node.next = node_to_delete.next;
        //        }

        //        return Some(node_to_delete.data);
        //    }
        //}

        None
    }

    fn search(&self, value: i32) -> Option<i32> {
        let mut current = self.head.as_ref();
        let mut result: Option<i32> = None;

        while let Some(node) = current {
            println!("in loop");
            if node.data == value {
                result = Some(value);
                break;
            }

            if node.next.is_none() {
                break;
            }

            current = node.next.as_ref();
        }

        match result {
            Some(result) => Some(result),
            None => None,
        }
    }
}

fn main() {
    let mut linked_list = LinkedList::new();

    for i in 1..=10 {
        linked_list.insert_at_end(i);
    }

    linked_list.search(5);

    println!("{:?}", linked_list);
}

#[cfg(test)]
#[test]
fn test_empty_constructor() {
    assert_eq!(
        LinkedList::new(),
        LinkedList {
            head: None,
            size: 0,
        }
    );
}

#[test]
fn test_insert_single_at_beginning() {
    let mut linked_list = LinkedList::new();

    linked_list.insert_at_beginning(1);

    assert_eq!(
        linked_list,
        LinkedList {
            head: Some(Box::new(Node {
                data: 1,
                next: None
            })),
            size: 1
        }
    );
}

#[test]
fn test_insert_double_at_beginning() {
    let mut linked_list = LinkedList::new();

    linked_list.insert_at_beginning(1);
    linked_list.insert_at_beginning(2);

    assert_eq!(
        linked_list,
        LinkedList {
            head: Some(Box::new(Node {
                data: 2,
                next: Some(Box::new(Node {
                    data: 1,
                    next: None
                }))
            })),
            size: 2
        }
    );
}

#[test]
fn test_insert_single_at_end() {
    let mut linked_list = LinkedList::new();

    linked_list.insert_at_end(1);

    assert_eq!(
        linked_list,
        LinkedList {
            head: Some(Box::new(Node {
                data: 1,
                next: None
            })),
            size: 1
        }
    );
}

#[test]
fn test_insert_double_at_end() {
    let mut linked_list = LinkedList::new();

    linked_list.insert_at_end(1);
    linked_list.insert_at_end(2);

    assert_eq!(
        linked_list,
        LinkedList {
            head: Some(Box::new(Node {
                data: 1,
                next: Some(Box::new(Node {
                    data: 2,
                    next: None
                }))
            })),
            size: 2
        }
    );
}

#[test]
fn test_search_single_node_list() {
    let mut linked_list = LinkedList::new();

    linked_list.insert_at_end(1);

    assert_eq!(linked_list.search(1), Some(1));
}

#[test]
fn test_search_multiple_node_list() {
    let mut linked_list = LinkedList::new();

    for i in 1..=10 {
        linked_list.insert_at_end(i);
    }

    assert_eq!(linked_list.search(5), Some(5));
}
