//! Awesome compatibilty modules
use rlua::{self, Lua};
pub mod keygrabber;
pub mod mousegrabber;
pub mod awful;
mod awesome;
mod client;
mod screen;
mod button;
mod tag;
mod key;
mod drawin;
mod drawable;
mod mouse;
mod root;
mod signal;
mod object;
mod class;
mod property;

pub use self::object::Object;
pub use self::keygrabber::keygrabber_handle;
pub use self::mousegrabber::mousegrabber_handle;

use std::env;
use std::path::PathBuf;

pub fn init(lua: &Lua) -> rlua::Result<()> {
    set_up_awesome_path(lua)?;
    button::init(lua)?.table;
    awesome::init(lua)?;
    key::init(lua)?;
    client::init(lua)?;
    screen::init(lua)?;
    keygrabber::init(lua)?;
    root::init(lua)?;
    mouse::init(lua)?;
    tag::init(lua)?;
    drawin::init(lua)?;
    drawable::init(lua)?;
    mousegrabber::init(lua)?;
    awful::init(lua)?;
    Ok(())
}

fn set_up_awesome_path(lua: &Lua) -> rlua::Result<()> {
    let globals = lua.globals();
    let package: rlua::Table = globals.get("package")?;
    //let paths: String = package.get("path")?;
    // TODO Do this right, I'm too lazy and just scrapped from my awesome env
    let mut path = package.get::<_, String>("path")?;
    let mut cpath = package.get::<_, String>("cpath")?;
    let mut xdg_data_path: PathBuf = env::var("XDG_DATA_DIRS").unwrap_or("/usr/share".into()).into();
    xdg_data_path.push("awesome/lib");
    path.push_str(&format!(";{0}/?.lua;{0}/?/init.lua",
                             xdg_data_path.as_os_str().to_string_lossy()));
    package.set("path", path)?;
    let mut xdg_config_path: PathBuf = env::var("XDG_CONFIG_DIRS").unwrap_or("/etc/xdg".into()).into();
    xdg_config_path.push("awesome");
    cpath.push_str(&format!(";{}/?.so;{}/?.so",
                            xdg_config_path.into_os_string().to_string_lossy(),
                            xdg_data_path.into_os_string().to_string_lossy()));
    package.set("cpath", cpath)?;

    // TODO Real debug, bug in rlua
    let debug = lua.create_table();
    debug.set("getinfo", lua.create_function(dummy_getinfo))?;
    debug.set("traceback", lua.create_function(dummy))?;
    globals.set("debug", debug)
}

// TODO Remove this and actually load the unsafe debug lib
fn dummy_getinfo<'lua>(lua: &'lua Lua, _: rlua::Value) -> rlua::Result<rlua::Table<'lua>> {
    fn gsub<'lua>(_: &'lua Lua, _: rlua::Value) -> rlua::Result<String> {
        Ok("FIXME Install debug lib!".into())
    }

    let table = lua.create_table();
    let call_table = lua.create_table();
    call_table.set("gsub", lua.create_function(gsub))?;
    table.set("source", call_table)?;
    Ok(table)
}

pub fn dummy<'lua>(_: &'lua Lua, _: rlua::Value) -> rlua::Result<()> { Ok(()) }
