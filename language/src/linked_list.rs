type NodePointer = Box<LinkedListNode>;

struct LinkedList {
    head: NodePointer,
}

struct LinkedListNode {
    key: i64,
    next: Option<NodePointer>,
}

impl LinkedList {
    fn new() -> LinkedList {
        return Self {
            head: Box::new(LinkedListNode { key: -1, next: None })
        };
    }

    fn append(&mut self, key: i64) {
        self.head.append(key);
    }

    fn has(&self, key: i64) -> bool {
        return self.head.has(key);
    }

    fn count(&self) -> u32 {
        return self.head.count();
    }
}

impl LinkedListNode {
    fn append(&mut self, key: i64) {
        match &mut self.next {
            None => {
                let node = LinkedListNode { key, next: None };
                self.next = Some(Box::new(node));
            }
            Some(ref mut following_node) => {
                following_node.append(key);
            }
        }
    }
    fn has(&self, key: i64) -> bool {
        if self.key == key {
            return true;
        }
        return match &self.next {
            None => {
                false
            }
            Some(following_node) => {
                following_node.has(key)
            }
        };
    }

    fn count(&self) -> u32 {
        return self._count(0);
    }

    fn _count(&self, count: u32) -> u32 {
        return match &self.next {
            None => {
                count
            }
            Some(following_node) => {
                following_node._count(count+1)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_ensure_a_key_is_present() {
        let mut list = LinkedList::new();
        list.append(10);
        list.append(30);
        list.append(20);

        let has = list.has(10);
        assert!(has);
    }

    #[test]
    fn test_add_and_check_for_a_missing_key() {
        let mut list = LinkedList::new();
        list.append(10);
        list.append(30);
        list.append(20);

        let has = list.has(2);
        assert_eq!(false, has);
    }

    #[test]
    fn test_add_and_count_for_single_key() {
        let mut list = LinkedList::new();
        list.append(10);

        let count = list.count();
        assert_eq!(1, count);
    }

    #[test]
    fn test_add_and_count_for_keys() {
        let mut list = LinkedList::new();
        list.append(10);
        list.append(30);
        list.append(20);
        list.append(90);
        list.append(80);

        let count = list.count();
        assert_eq!(5, count);
    }

}

