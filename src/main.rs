use rlua::{Lua};
use rlua_async::{ChunkExt, ContextExt};

#[actix_rt::main]
async fn main() {
    let lua_code = "my.asyncfunc(42)";
    let lua = Lua::new();

    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        let map_table = lua_ctx.create_table().unwrap();
        map_table
            .set(
                "asyncfunc",
                lua_ctx
                    .create_async_function(
                        |_ctx,
                         param:
                            u32
                        | async move {
                            println!("async function called {}", param);
                            Ok(())
                        }).unwrap()).unwrap();

        globals.set("my", map_table).unwrap();
    });

    lua.context(|lua_context| async move {
        let chunk = lua_context
            .load(&lua_code);
        chunk.exec_async(lua_context).await.unwrap();
    })
        .await;
    println!("finished");
}
