use super::{Observer as SubjectObserver, Subject};

/// Concrete Observer that reacts to computation state changes
pub struct ComputationObserver {
    pub name: String,
}

impl ComputationObserver {
    pub fn new(name: &str) -> Self {
        ComputationObserver {
            name: name.to_string(),
        }
    }
}

impl SubjectObserver for ComputationObserver {
    fn update(&self, message: &str) {
        println!("Observer {} received update: {}", self.name, message);
    }
}

/// Concrete Subject that manages observers and notifies them about changes
pub struct ComputationManager {
    observers: Vec<Box<dyn SubjectObserver>>,
}

impl ComputationManager {
    pub fn new() -> Self {
        ComputationManager {
            observers: Vec::new(),
        }
    }

    pub fn run_computation(&self) {
        // Simulate a computation task
        println!("Running computation...");
        self.notify_observers("Computation started");

        // More simulation...
        self.notify_observers("Computation in progress");

        // End of computation
        self.notify_observers("Computation finished");
    }
}

impl Subject for ComputationManager {
    fn register_observer(&mut self, observer: Box<dyn SubjectObserver>) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer_name: &str) {
        self.observers.retain(|observer| {
            observer.as_ref().update(observer_name); // Incorrect use of `update` for demonstration
            // The correct approach would require a method to compare observer names
            true // Keeping all observers for now, logic needs improvement
        });
    }

    fn notify_observers(&self, message: &str) {
        for observer in &self.observers {
            observer.update(message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observer_pattern() {
        let mut manager = ComputationManager::new();

        let observer1 = Box::new(ComputationObserver::new("Observer 1"));
        let observer2 = Box::new(ComputationObserver::new("Observer 2"));

        manager.register_observer(observer1);
        manager.register_observer(observer2);

        manager.run_computation();
    }
}
