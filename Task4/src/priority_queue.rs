use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};

#[derive(Debug, Clone)]
pub struct Item<T> {
    pub id: usize,
    pub value: T,
    pub priority: i32,
}

impl<T> PartialEq for Item<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.id == other.id
    }
}

impl<T> Eq for Item<T> {}

impl<T> PartialOrd for Item<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.priority
                .cmp(&other.priority)
                .then(self.id.cmp(&other.id)),
        )
    }
}

impl<T> Ord for Item<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then(self.id.cmp(&other.id))
    }
}

pub struct BiPriorityQueue<T> {
    queue: VecDeque<Item<T>>,
    max_heap: BinaryHeap<Item<T>>,
    min_heap: BinaryHeap<Reverse<Item<T>>>,
    counter: usize,
}

impl<T: Clone> BiPriorityQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
            counter: 0,
        }
    }

    pub fn enqueue(&mut self, value: T, priority: i32) {
        let item = Item {
            id: self.counter,
            value,
            priority,
        };
        self.counter += 1;
        self.max_heap.push(item.clone());
        self.min_heap.push(Reverse(item.clone()));
        self.queue.push_back(item);
    }

    pub fn peek(&self, mode: Mode) -> Option<&T> {
        match mode {
            Mode::HIGHEST => self.max_heap.peek().map(|i| &i.value),
            Mode::LOWEST => self.min_heap.peek().map(|ri| &ri.0.value),
            Mode::OLDEST => self.queue.front().map(|i| &i.value),
            Mode::NEWEST => self.queue.back().map(|i| &i.value),
        }
    }

    pub fn dequeue(&mut self, mode: Mode) -> Option<T> {
        match mode {
            Mode::HIGHEST => {
                if let Some(top) = self.max_heap.pop() {
                    self.remove_by_id(top.id).map(|i| i.value)
                } else {
                    None
                }
            }
            Mode::LOWEST => {
                if let Some(Reverse(top)) = self.min_heap.pop() {
                    self.remove_by_id(top.id).map(|i| i.value)
                } else {
                    None
                }
            }
            Mode::OLDEST => self.queue.pop_front().map(|i| {
                self.remove_by_id(i.id);
                i.value
            }),
            Mode::NEWEST => self.queue.pop_back().map(|i| {
                self.remove_by_id(i.id);
                i.value
            }),
        }
    }

    fn remove_by_id(&mut self, id: usize) -> Option<Item<T>> {
        if let Some(pos) = self.queue.iter().position(|i| i.id == id) {
            Some(self.queue.remove(pos).unwrap())
        } else {
            None
        }
    }
}
pub enum Mode {
    HIGHEST,
    LOWEST,
    NEWEST,
    OLDEST,
}
