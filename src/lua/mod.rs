use mlua::{prelude::*, Lua, UserData};
use raylib::*;
use std::{thread, time};

use crate::function_name;

fn used_memory(lua: &Lua, _: ()) -> LuaResult<usize> {
    Ok(lua.used_memory())
}

fn gc_reset(lua: &Lua, _: ()) -> LuaResult<()> {
    lua.gc_restart();
    Ok(())
}

fn wait(lua: &Lua, time: u64) -> LuaResult<()> {
    thread::sleep(time::Duration::from_secs(time));
    Ok(())
}

#[derive(Debug)]
struct LuaRaylibHandle(RaylibHandle);

#[derive(Debug)]
struct LuaRaylib((LuaRaylibHandle, LuaRaylibThread));
impl UserData for LuaRaylib {}
#[derive(Clone, Debug)]
struct LuaRaylibThread(RaylibThread);

impl UserData for LuaRaylibThread {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(_methods: &mut M) {}
}

impl UserData for LuaRaylibHandle {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("get_fps", |_, this| Ok(this.0.get_fps()));

        fields.add_field_method_get("window_should_close", |_, this| {
            Ok(this.0.window_should_close())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        //methods.add_method("begin_drawing", |_, this, value: LuaRaylibThread| {
        //    return
        //})
        methods.add_method_mut("set_target_fps", |_, this, value: u32| {
            Ok(this.0.set_target_fps(value))
        })
    }
}

fn window_create(lua: &Lua, (width, height, name): (i32, i32, String)) -> LuaResult<LuaRaylib> {
    let rl = raylib::init()
        .size(width, height)
        .title(name.as_str())
        .build();
    let _rl = (LuaRaylibHandle(rl.0), LuaRaylibThread(rl.1));
    Ok(LuaRaylib(_rl))
}

fn mass_import<'a, T>(lua: &'a Lua, misc: &[Fn<(&Lua, T)>]) {
    let globals = lua.globals();
    misc.into_iter()
        .map(|x| globals.set(function_name!(x), lua.create_function(x)?));
}

fn import_libs(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();
    let misc = lua.create_table()?;
    misc.set("used_memory", lua.create_function(used_memory)?)?;
    misc.set("gc_restart", lua.create_function(gc_reset)?)?;
    globals.set("misc", misc)?;

    let window = lua.create_table()?;
    window.set("create", lua.create_function(window_create)?)?;
    globals.set("window", window)?;
    mass_import(lua, &[gc_reset]);
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
