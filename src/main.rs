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
        let mut deleted_record = None;

        // Handle first case
        if let Some(node) = &self.head {
            if node.data == value {
                let old_head = self.head.take()?;
                self.head = old_head.next;
                self.size -= 1;
                deleted_record = Some(old_head.data);
            }
        }

        let mut current = self.head.as_mut();

        while let Some(node) = current {
            if let Some(next) = node.next.as_mut() {
                if next.data == value {
                    deleted_record = Some(next.data);
                    node.next = next.next.take();
                    self.size -= 1;
                }
            }

            current = node.next.as_mut();
        }

        deleted_record
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

    linked_list.insert_at_end(5);

    linked_list.delete(5);

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

#[test]
fn test_delete_multiple_first_node_in_list() {
    let mut linked_list = LinkedList::new();

    for i in 1..=3 {
        linked_list.insert_at_end(i);
    }

    linked_list.delete(1);

    assert_eq!(
        linked_list,
        LinkedList {
            size: 2,
            head: Some(Box::new(Node {
                data: 2,
                next: Some(Box::new(Node {
                    data: 3,
                    next: None
                })),
            })),
        }
    );
}

#[test]
fn test_delete_multiple_middle_node_in_list() {
    let mut linked_list = LinkedList::new();

    for i in 1..=3 {
        linked_list.insert_at_end(i);
    }

    linked_list.delete(2);

    assert_eq!(
        linked_list,
        LinkedList {
            size: 2,
            head: Some(Box::new(Node {
                data: 1,
                next: Some(Box::new(Node {
                    data: 3,
                    next: None
                })),
            })),
        }
    );
}

#[test]
fn test_delete_multiple_last_node_in_list() {
    let mut linked_list = LinkedList::new();

    for i in 1..=3 {
        linked_list.insert_at_end(i);
    }

    linked_list.delete(3);

    assert_eq!(
        linked_list,
        LinkedList {
            size: 2,
            head: Some(Box::new(Node {
                data: 1,
                next: Some(Box::new(Node {
                    data: 2,
                    next: None
                })),
            })),
        }
    );
}

#[test]
fn test_delete_multiple_multiple_nodes_in_list() {
    let mut linked_list = LinkedList::new();

    for i in 1..=3 {
        linked_list.insert_at_end(i);
    }

    linked_list.insert_at_end(1);

    linked_list.delete(1);

    assert_eq!(
        linked_list,
        LinkedList {
            size: 2,
            head: Some(Box::new(Node {
                data: 2,
                next: Some(Box::new(Node {
                    data: 3,
                    next: None
                })),
            })),
        }
    );
}
