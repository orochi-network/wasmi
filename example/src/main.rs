use anyhow::Result;
use std::sync::{Arc, Mutex};
use wasmi::predator::Predator;
use wasmi::Module;
use wasmi::*;
use wat;

fn main() -> Result<()> {
    // First step is to create the Wasm execution engine with some config.
    // In this example we are using the default configuration.
    let wasm_predator = Arc::new(Mutex::new(Predator::new()));
    let engine = Engine::new_with_predator(wasm_predator.clone());
    let wat = r#"
    (module
        (func (export "add_values") (param i64 i64) (result i64)
          local.get 0
          local.get 1
          i64.add))
    "#;
    // Wasmi does not yet support parsing `.wat` so we have to convert
    // out `.wat` into `.wasm` before we compile and validate it.
    let wasm = wat::parse_str(&wat)?;
    let module = Module::new(&engine, &mut &wasm[..])?;

    // All Wasm objects operate within the context of a `Store`.
    // Each `Store` has a type parameter to store host-specific data,
    // which in this case we are using `42` for.
    type HostState = u32;
    let mut store = Store::new(&engine, 42);

    // In order to create Wasm module instances and link their imports
    // and exports we require a `Linker`.
    let linker = <Linker<HostState>>::new(&engine);
    // Instantiation of a Wasm module requires defining its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    //
    // Also before using an instance created this way we need to start it.
    // linker.define("host", "add_values", host_hello)?;
    let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;
    let add_function = instance.get_typed_func::<(i64, i64), i64>(&store, "add_values")?;

    // And finally we can call the wasm!
    let result = add_function
        .call(&mut store, (4, 5))
        .expect("Unable to execute function");
    println!("Function result is: {}", result);
    let execution_trace = wasm_predator.clone().lock().unwrap().get_trace();
    for i in 0..execution_trace.len() {
        println!("{:#?}", execution_trace[i]);
    }
    Ok(())
}
