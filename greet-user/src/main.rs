use clap::Parser;
use wasmtime::{Engine, Store};
use wasmtime::component::{bindgen, Component, Instance, Linker, TypedFunc};

bindgen!({
    world: "greetable-provider",
    path: "/Users/tamurashigeki/Develop/Docker/wasm/wasm-practise/greet/wit",
});

#[derive(Parser, Debug)]
struct Args {
    wasm_file: String
}

fn start(args: Args) -> anyhow::Result<()> {
    let engine = Engine::default();

    let component = Component::from_file(&engine, &args.wasm_file)?;

    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    let provider= GreetableProvider::instantiate(&mut store, &component, &linker)?;

    let greetable = provider.reo0306_greet_greetable();

    let message = greetable.call_greet(&mut store, "world")?;
    println!("{message}");

    let name = greetable.call_name(&mut store)?;
    let message = greetable.call_greet(&mut store, &name)?;
    println!("{message}");

    Ok(())
/*
    let instance = linker.instantiate(&mut store, &component)?;

    let greetable_index = instance.get_export(
        &mut store,
        None,
        "reo0306:greet/greetable"
    ).unwrap();

    let greet_index = instance.get_export(&mut store, Some(&greetable_index), "greet").unwrap();

    let name_index = instance.get_export(&mut store, Some(&greetable_index), "name").unwrap();

    let greet: TypedFunc<(String, ), (String, )> = instance.get_typed_func(&mut store, greet_index).unwrap();

    let name: TypedFunc<(), (String, )> = instance.get_typed_func(&mut store, name_index).unwrap();

    let argument = "world!".to_string();

    let (return_value, ) = greet.call(&mut store, (argument, ))?;

    greet.post_return(&mut store)?;

    println!("{return_value}");

    let (returned_name, ) = name.call(&mut store, ())?;
    name.post_return(&mut store)?;

    let (return_value, ) = greet.call(&mut store, (returned_name, ))?;
    greet.post_return(&mut store)?;

    println!("{return_value}");

    Ok(())
*/
}

fn main() {
    let args = Args::parse();

    if let Err(e) = start(args) {
        println!("{:?}", e);
    }
}
