use rlua::Lua;
use rlua_async::{ChunkExt, ContextExt};
const LUA_CODE: &str = "my.asyncfunc(42)";
#[actix_rt::main]
async fn main() -> rlua::Result<()> {
    let lua = Lua::new();
    lua.context::<_, rlua::Result<()>>(|lua_ctx| {
        let globals = lua_ctx.globals();
        let map_table = lua_ctx.create_table()?;
        map_table.set(
            "asyncfunc",
            lua_ctx.create_async_function(|_ctx, param: u32| async move {
                println!("async function called {}", param);
                Ok(())
            })?,
        )?;
        globals.set("my", map_table)?;
        Ok(())
    });
    lua.context(|lua_context| {
        actix::System::new("Default")
            .block_on(lua_context.load(LUA_CODE).exec_async(lua_context))
            .unwrap();
    });
    Ok(())
}
