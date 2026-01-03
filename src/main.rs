use std::mem;

enum Slot<T> {
    Free(Option<usize>),
    Used(T)
}

impl<T> Slot<T> {
    fn as_free(&self) -> &Option<usize> {
        match self {
            Slot::Free(pos) => pos,
            _ => panic!("expected free slot")
        }
    }

    fn as_free_mut(&mut self) -> &mut Option<usize> {
        match self {
            Slot::Free(pos) => pos,
            _ => panic!("expected free slot")
        }
    }

    fn into_free(self) -> Option<usize> {
        match self {
            Slot::Free(pos) => pos,
            _ => panic!("expected free slot")
        }
    }

    fn as_used(&self) -> &T {
        match self {
            Slot::Used(val) => val,
            _ => panic!("expected used slot")
        }
    }

    fn as_used_mut(&mut self) -> &mut T {
        match self {
            Slot::Used(val) => val,
            _ => panic!("expected used slot")
        }
    }

    fn into_used(self) -> T {
        match self {
            Slot::Used(val) => val,
            _ => panic!("expected used slot")
        }
    }
}

struct LinkedListNode<T> {
    prev: Option<usize>,
    next: Option<usize>,
    val: T
}

struct LinkedList<T> {
    size: usize,
    head: Option<usize>,
    tail: Option<usize>,
    free: Option<usize>,
    slots: Vec<Slot<LinkedListNode<T>>>
}

struct LinkedListIterator<'a, T> {
    list: &'a LinkedList<T>,
    curr: Option<usize>
}

impl<'a, T> LinkedListIterator<'a, T> {
    fn new(list: &'a LinkedList<T>) -> LinkedListIterator<'a, T> {
        LinkedListIterator { list, curr: list.head }
    }
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(pos) = self.curr else {
            return None;
        };

        let node = self.list.slots[pos].as_used();

        self.curr = node.next;

        Some(&node.val)
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = LinkedListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            size: 0,
            head: None,
            tail: None,
            free: None,
            slots: Vec::new()
        }
    }

    pub fn iter(&self) -> LinkedListIterator<'_, T> {
        LinkedListIterator::new(self)
    }

    pub fn size(&self) -> usize {
        return self.size
    }

    pub fn is_empty(&self) -> bool {
        return self.head.is_none();
    }

    pub fn get_first(&self) -> Option<&T> {
        self.head.map(|pos| &self.slots[pos].as_used().val)
    }

    pub fn get_last(&self) -> Option<&T> {
        self.tail.map(|pos| &self.slots[pos].as_used().val)
    }

    pub fn add_first(&mut self, val: T) {
        let node = LinkedListNode {
            prev: None,
            next: self.head,
            val
        };

        let new_head = self.insert(node);

        match self.head {
            None => {
                self.head = Some(new_head);
                self.tail = Some(new_head);
            },
            Some(old_head) => {
                self.slots[old_head].as_used_mut().prev = Some(new_head);
                self.head = Some(new_head);
            }
        }
    }

    pub fn add_last(&mut self, val: T) {
        let node = LinkedListNode {
            prev: self.tail,
            next: None,
            val
        };

        let new_tail = self.insert(node);

        match self.tail {
            None => {
                self.head = Some(new_tail);
                self.tail = Some(new_tail);
            },
            Some(old_tail) => {
                self.slots[old_tail].as_used_mut().next = Some(new_tail);
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn remove_first(&mut self) -> Option<T> {
        self.head.map(|pos| {
            self.head = self.slots[pos].as_used().next;

            match self.head {
                None => {
                    self.tail = None;
                }
                Some(new_head) => {
                    self.slots[new_head].as_used_mut().prev = None;
                }
            }

            let slot = mem::replace(
                &mut self.slots[pos],
                Slot::Free(self.free)
            );

            self.free = Some(pos);
            self.size -= 1;

            slot.into_used().val
        })
    }

    pub fn remove_last(&mut self) -> Option<T> {
        self.tail.map(|pos| {
            self.tail = self.slots[pos].as_used().prev;

            match self.tail {
                None => {
                    self.head = None;
                }
                Some(new_tail) => {
                    self.slots[new_tail].as_used_mut().next = None;
                }
            }

            let slot = mem::replace(
                &mut self.slots[pos],
                Slot::Free(self.free)
            );

            self.free = Some(pos);
            self.size -= 1;

            slot.into_used().val
        })
    }

    fn insert(&mut self, node: LinkedListNode<T>) -> usize {
        let slot = Slot::Used(node);

        self.size += 1;

        match self.free {
            None => {
                self.slots.push(slot);
                self.slots.len() - 1
            },
            Some(curr) => {
                self.free = *self.slots[curr].as_free();
                self.slots[curr] = slot;
                curr
            }
        }
    }
}

fn main() {
    let mut list: LinkedList<&str> = LinkedList::new();

    list.add_last("Hello");
    list.add_last("World");

    for item in &list {
        println!("{item}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list_behaviour() {
        let mut list: LinkedList<i32> = LinkedList::new();

        assert!(list.is_empty());
        assert_eq!(0, list.size());
        assert_eq!(None, list.remove_first());
        assert_eq!(None, list.remove_last());
    }

    #[test]
    fn add_and_remove_first_last() {
        let mut list = LinkedList::new();

        list.add_last(1);
        list.add_last(2);
        list.add_first(0);

        let mut it = list.iter();

        assert_eq!(it.next(), Some(&0));
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), None);

        assert_eq!(Some(0), list.remove_first());
        assert_eq!(Some(2), list.remove_last());
        assert_eq!(Some(1), list.remove_first());
        assert_eq!(None, list.remove_first());
    }

    #[test]
    fn free_slot_reuse() {
        let mut list = LinkedList::new();

        list.add_last(10);
        list.add_last(20);
        list.add_last(30);

        assert_eq!(Some(10), list.remove_first());
        assert_eq!(Some(20), list.remove_first());

        list.add_last(40);
        list.add_first(0);

        let vals: Vec<_> = list.iter().copied().collect();

        assert_eq!(vals, vec![0, 30, 40]);
    }
}
