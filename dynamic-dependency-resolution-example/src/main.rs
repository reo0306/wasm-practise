use clap::Parser;
use wasmtime::{Engine, Store};
use wasmtime::component::{bindgen, Component, Linker};

use reo0306::greet::greetable::Host;

bindgen!({
    path: "/Users/tamurashigeki/Develop/Docker/wasm/wasm-practise/greet/wit",
    world: "hello-world",
});

#[derive(Parser, Debug)]
struct Cli {
    wasm_file: String,
}

struct Greet {
    name: String,
}

impl Greet {
    fn new(name: String) -> Greet {
        Greet { name }
    }
}

impl Host for Greet {
    fn name(&mut self) -> String {
        self.name.clone()
    }

    fn greet(&mut self, name: String) -> String {
        format!("Hello from {name}")
    }
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = start(cli) {
        println!("{e}");
    }
}

fn start(cli: Cli) -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, Greet::new("Native code".to_string()));

    let component = Component::from_file(&engine, &cli.wasm_file)?;

    HelloWorld::add_to_linker(&mut linker, |greet: &mut Greet| greet)?;

    let hello_world = HelloWorld::instantiate(&mut store, &component, &linker)?;

    let message = hello_world.reo0306_greet_sayable().call_say(&mut store)?;
    println!("{message}");

    Ok(())
}
