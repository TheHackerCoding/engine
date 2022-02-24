use mlua::Lua;
pub mod wait;
use crate::lua::functions::wait::wait;

pub fn mass_import(lua: &Lua) {
    let globals = lua.globals();
    globals.set("wait", wait);
}
