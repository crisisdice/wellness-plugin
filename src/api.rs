use rhai_dylib::rhai::plugin::{
    mem, Dynamic, FnAccess, FnNamespace, ImmutableString, NativeCallContext, PluginFunction,
    RhaiResult, TypeId,
};

use rhai_dylib::rhai::Module;

// The plugin API from rhai can be used to create your plugin API.
#[rhai_dylib::rhai::plugin::export_module]
pub mod wellness_plugin {
    /// Printing to the console using Rust.
    #[rhai_fn(global)]
    pub fn print_stuff(string: String) {
        println!("{}", string);
    }
}
