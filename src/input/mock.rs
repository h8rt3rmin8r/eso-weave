//! A test backend that captures synthesized output and lets tests drive
//! classification without operating-system hooks.

use std::sync::{Arc, Mutex};

use crate::input::{InputBackend, InputEngine, InputError, Key, MouseButton, Transition};

/// A test double for [`InputBackend`]. Records synthesized key and mouse
/// transitions so tests can assert on the engine's output; `run` is a no-op
/// because there is no real interception loop.
#[derive(Default)]
pub struct MockBackend {
    /// The key transitions synthesized through this backend, in order.
    pub synthesized: Arc<Mutex<Vec<(Key, Transition)>>>,
    /// The mouse transitions synthesized through this backend, in order.
    pub synthesized_mouse: Arc<Mutex<Vec<(MouseButton, Transition)>>>,
}

impl MockBackend {
    /// Creates an empty mock backend.
    pub fn new() -> Self {
        Self::default()
    }

    /// A snapshot of the synthesized key transitions so far.
    pub fn synthesized(&self) -> Vec<(Key, Transition)> {
        self.synthesized.lock().unwrap().clone()
    }

    /// A snapshot of the synthesized mouse transitions so far.
    pub fn synthesized_mouse(&self) -> Vec<(MouseButton, Transition)> {
        self.synthesized_mouse.lock().unwrap().clone()
    }
}

impl InputBackend for MockBackend {
    fn synthesize(&self, key: Key, transition: Transition) -> Result<(), InputError> {
        self.synthesized.lock().unwrap().push((key, transition));
        Ok(())
    }

    fn synthesize_mouse(
        &self,
        button: MouseButton,
        transition: Transition,
    ) -> Result<(), InputError> {
        self.synthesized_mouse
            .lock()
            .unwrap()
            .push((button, transition));
        Ok(())
    }

    fn run(&self, _engine: Arc<InputEngine>) -> Result<(), InputError> {
        Ok(())
    }
}
