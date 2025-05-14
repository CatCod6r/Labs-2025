mod priority_queue;
use priority_queue::{BiPriorityQueue, Mode};

fn main() {
    let mut queue = BiPriorityQueue::new();

    queue.enqueue("apple", 5);
    queue.enqueue("banana", 2);
    queue.enqueue("orange", 10);
    queue.enqueue("grape", 7);

    println!("Peek highest: {:?}", queue.peek(Mode::HIGHEST));
    println!("Peek lowest: {:?}", queue.peek(Mode::LOWEST));
    println!("Peek oldest: {:?}", queue.peek(Mode::OLDEST));
    println!("Peek newest: {:?}", queue.peek(Mode::NEWEST));

    println!("Dequeue highest: {:?}", queue.dequeue(Mode::HIGHEST));
    println!("Dequeue lowest: {:?}", queue.dequeue(Mode::LOWEST));
    println!("Dequeue oldest: {:?}", queue.dequeue(Mode::OLDEST));
    println!("Dequeue newest: {:?}", queue.dequeue(Mode::NEWEST));
}
