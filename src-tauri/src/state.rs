use std::process::Child;
use std::sync::Mutex;

// A thread-safe wrapper around the child process handle.
// We use Option because the node might not be running.
pub struct NodeState {
    pub process: Mutex<Option<Child>>,
}

impl NodeState {
    pub fn new() -> Self {
        Self {
            process: Mutex::new(None),
        }
    }
}
