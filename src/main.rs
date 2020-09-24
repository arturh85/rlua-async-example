use rlua::Lua;
use rlua_async::{ChunkExt, ContextExt};

fn main() -> rlua::Result<()> {
    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        let map_table = ctx.create_table()?;
        map_table.set(
            "asyncfunc",
            ctx.create_async_function(|_ctx, param: u32| async move {
                println!("async function called {}", param);
                Ok(())
            })?,
        )?;
        globals.set("my", map_table)?;
        let chunk = ctx.load("my.asyncfunc(42)");
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(chunk.exec_async(ctx))
    })?;
    Ok(())
}
