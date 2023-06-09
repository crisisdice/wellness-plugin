use rhai_dylib::rhai::{config::hashing::set_ahash_seed, ImmutableString};
use rhai_dylib::{module_resolvers::libloading::DylibModuleResolver, rhai};

fn main() {
    if let Err(value) = set_ahash_seed(Some([1, 2, 3, 4])) {
        panic!("ahash seed has been overridden by a plugin: {value:?}");
    }

    println!("main: {:?}", std::any::TypeId::of::<ImmutableString>());

    let mut engine = rhai::Engine::new();

    engine.set_module_resolver(DylibModuleResolver::new());

    engine
        .run(
            r#"
import "./target/debug/libwellness_plugin" as plugin;

plugin::print_stuff();
"#,
        )
        .expect("failed to execute script");
}
