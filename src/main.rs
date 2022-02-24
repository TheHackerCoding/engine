use lua::LuaInstance;
use std::{env, fs, panic::panic_any};
mod utils;
use utils::exit;

mod lua;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source_file = match args.get(1) {
        Some(s) => s,
        None => panic_any("Unable to get lua source file"),
    };
    let source = fs::read_to_string(source_file).expect("Unable to read file.");
    let lua = LuaInstance::init();
    //let globals = lua.instance.globals();
    //let print: Function = globals.get("print").unwrap();
    //print.call::<_, ()>("hello from rust").unwrap();
    lua.instance
        //.load("print(misc.used_memory()); window.create(500,500,'hi')")
        .load(&source)
        .exec()
        .unwrap();
}
