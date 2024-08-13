use super::Plugin;

/// A sample plugin to demonstrate how plugins work.
pub struct SamplePlugin;

impl SamplePlugin {
    /// Creates a new SamplePlugin instance.
    pub fn new() -> Self {
        SamplePlugin
    }
}

impl Plugin for SamplePlugin {
    fn name(&self) -> String {
        "Sample Plugin".to_string()
    }

    fn execute(&self) {
        println!("Sample Plugin executed!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_plugin_name() {
        let plugin = SamplePlugin::new();
        assert_eq!(plugin.name(), "Sample Plugin");
    }

    #[test]
    fn test_sample_plugin_execute() {
        let plugin = SamplePlugin::new();
        plugin.execute(); // Should print "Sample Plugin executed!"
    }
}
