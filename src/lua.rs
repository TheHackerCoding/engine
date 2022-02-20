use mlua::{prelude::*, AnyUserData, Lua, Table, UserData};
use raylib::*;

fn used_memory(lua: &Lua, _: ()) -> LuaResult<usize> {
    Ok(lua.used_memory())
}

fn gc_reset(lua: &Lua, _: ()) -> LuaResult<usize> {
    lua.gc_restart();
    Ok(1)
}
struct LuaRaylibHandle(RaylibHandle);

struct LuaRaylibThread(RaylibThread);

impl LuaRaylibThread {}

impl UserData for LuaRaylibHandle {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("get_fps", |_, this| Ok(this.0.get_fps()));

        fields.add_field_method_get("window_should_close", |_, this| {
            Ok(this.0.window_should_close())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("begin_drawing", |_, this, value: RaylibThread| {})
    }
}

fn window_create(
    lua: &Lua,
    (width, height, name): (i32, i32, String),
) -> LuaResult<(LuaRaylibHandle, LuaRaylibThread)> {
    let rl = raylib::init()
        .size(width, height)
        .title(name.as_str())
        .build();
    let _rl = (LuaRaylibHandle(rl.0), LuaRaylibThread(rl.1));
    Ok(_rl)
}

fn import_libs(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();
    let misc = lua.create_table()?;
    misc.set("used_memory", lua.create_function(used_memory)?)?;
    misc.set("gc_restart", lua.create_function(gc_reset)?)?;
    globals.set("misc", misc)?;

    let window = lua.create_table()?;
    window.set("create", lua.create_function(window_create)?);
    globals.set("window", window)?;
    Ok(())
}

pub struct LuaInstance {
    pub instance: Lua,
}

impl LuaInstance {
    pub fn init() -> LuaInstance {
        let lua = Lua::new();
        import_libs(&lua).unwrap();
        LuaInstance { instance: lua }
    }
}
