use types::Task;
use types::graph::Graph;

pub type TaskList<K> = Graph<K, Box<Task>, ()>;
