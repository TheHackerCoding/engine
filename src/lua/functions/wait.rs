use mlua::{prelude::LuaResult, Lua};
use std::{thread, time};

pub fn wait(_: &Lua, time: u64) -> LuaResult<()> {
    thread::sleep(time::Duration::from_secs(time));
    Ok(())
}
