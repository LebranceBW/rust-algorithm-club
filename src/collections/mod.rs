//! Colletions are general-purpose or specialized data structures in which elements store.

mod singly_linked_list;
pub use self::singly_linked_list::SinglyLinkedList;

mod hash_map;
pub use self::hash_map::HashMap;

mod set;
pub use self::set::HashSet;

mod stack;
pub use self::stack::Stack;

mod bloom_filter;
pub use self::bloom_filter::BloomFilter;

mod deque;
pub use self::deque::Deque;

mod queue;
pub use self::queue::Queue;

mod skip_list;
pub use self::skip_list::SkipList;
