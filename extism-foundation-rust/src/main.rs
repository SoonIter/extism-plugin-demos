use anyhow::Error;
use extism::{Context, CurrentPlugin, Function, Plugin, UserData, Val, ValType};

fn hello_world(
    _plugin: &mut CurrentPlugin,
    inputs: &[Val],
    outputs: &mut [Val],
    _user_data: UserData,
) -> Result<(), Error> {
    println!("Hello from Rust!");
    outputs[0] = inputs[0].clone();
    Ok(())
}

fn main() {
    let wasm = include_bytes!(
        "../../extism-plugin-rust/target/wasm32-unknown-unknown/release/extism_plugin_rust.wasm"
    );
    let wasm2 = include_bytes!(
        "./code.wasm"
    );
    // let wasm = include_bytes!("hello.wasm");
    let context = Context::new();
    // NOTE: if you encounter an error such as:
    // "Unable to load plugin: unknown import: wasi_snapshot_preview1::fd_write has not been defined"
    // change `false` to `true` in the following function to provide WASI imports to your plugin.

    let f = Function::new(
        "hello_world",
        [ValType::I64],
        [ValType::I64],
        None,
        hello_world
    );

    let functions = [&f];
    let mut plugin = Plugin::new(&context, wasm, functions, true).unwrap();
    let mut plugin2 = Plugin::new(&context, wasm2, functions, true).unwrap();
    let data = plugin.call("count_vowels", "this is a test").unwrap();
    println!("{:?}", String::from_utf8(data.to_owned()));
    assert_eq!(data, b"{\"count\":4,\"a\":\"this is var a\"}");

    let data = String::from_utf8(plugin2.call("count_vowels", "this is a test").unwrap().to_owned()).unwrap();
    println!("{:?}", data);
    assert_eq!(data, "{\"count\": 4}");
}
