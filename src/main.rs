use lua::LuaInstance;
use mlua::Function;

mod lua;

fn main() {
    let lua = LuaInstance::init();
    let globals = lua.instance.globals();
    let print: Function = globals.get("print").unwrap();
    print.call::<_, ()>("hello from rust").unwrap();
    lua.instance
        .load("print(misc.used_memory()); window.create(500,500,'hi')")
        .exec()
        .unwrap();
}
