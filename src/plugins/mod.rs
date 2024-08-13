// This module handles the plugin system, allowing dynamic loading of additional functionality.
pub mod sample_plugin;

// Trait that all plugins must implement
pub trait Plugin {
    fn name(&self) -> String;
    fn execute(&self);
}

// Function to load a plugin dynamically (placeholder implementation)
pub fn load_plugin(plugin_name: &str) -> Box<dyn Plugin> {
    match plugin_name {
        "sample_plugin" => Box::new(sample_plugin::SamplePlugin::new()),
        _ => panic!("Unknown plugin: {}", plugin_name),
    }
}
