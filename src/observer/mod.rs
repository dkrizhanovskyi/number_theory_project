// This module manages observers that track computation states and notify when changes occur.
pub mod computation_observer;

// Trait that defines the Observer interface
pub trait Observer {
    fn update(&self, message: &str);
}

// Trait that defines the Subject interface, which notifies observers
pub trait Subject {
    fn register_observer(&mut self, observer: Box<dyn Observer>);
    fn remove_observer(&mut self, observer_name: &str);
    fn notify_observers(&self, message: &str);
}
