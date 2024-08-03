use anyhow::{anyhow, Result};
use test_programs_artifacts::{foreach_keyvalue, KEYVALUE_MAIN_COMPONENT};
use wasmtime::{
    component::{Component, Linker, ResourceTable},
    Store,
};
use wasmtime_wasi::{bindings::Command, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_keyvalue::{WasiKeyValue, WasiKeyValueCtx, WasiKeyValueCtxBuilder};

struct Ctx {
    table: ResourceTable,
    wasi_ctx: WasiCtx,
    wasi_keyvalue_ctx: WasiKeyValueCtx,
}

impl WasiView for Ctx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

async fn run_wasi(path: &str, ctx: Ctx) -> Result<()> {
    let engine = test_programs_artifacts::engine(|config| {
        config.async_support(true);
    });
    let mut store = Store::new(&engine, ctx);
    let component = Component::from_file(&engine, path)?;

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_async(&mut linker)?;
    wasmtime_wasi_keyvalue::add_to_linker_async(&mut linker, |h: &mut Ctx| {
        WasiKeyValue::new(&h.wasi_keyvalue_ctx, &mut h.table)
    })?;

    let command = Command::instantiate_async(&mut store, &component, &linker).await?;
    command
        .wasi_cli_run()
        .call_run(&mut store)
        .await?
        .map_err(|()| anyhow!("command returned with failing exit status"))
}

macro_rules! assert_test_exists {
    ($name:ident) => {
        #[allow(unused_imports)]
        use self::$name as _;
    };
}

foreach_keyvalue!(assert_test_exists);

#[tokio::test(flavor = "multi_thread")]
async fn keyvalue_main() -> Result<()> {
    run_wasi(
        KEYVALUE_MAIN_COMPONENT,
        Ctx {
            table: ResourceTable::new(),
            wasi_ctx: WasiCtxBuilder::new()
                .inherit_stderr()
                .env("IDENTIFIER", "")
                .build(),
            wasi_keyvalue_ctx: WasiKeyValueCtxBuilder::new()
                .in_memory_data([("atomics_key", "5")])
                .build(),
        },
    )
    .await
}

#[cfg(feature = "redis")]
#[tokio::test(flavor = "multi_thread")]
async fn keyvalue_redis() -> Result<()> {
    run_wasi(
        KEYVALUE_MAIN_COMPONENT,
        Ctx {
            table: ResourceTable::new(),
            wasi_ctx: WasiCtxBuilder::new()
                .inherit_stderr()
                .env("IDENTIFIER", "redis://127.0.0.1/")
                .build(),
            wasi_keyvalue_ctx: WasiKeyValueCtxBuilder::new()
                .allow_redis_hosts(&["127.0.0.1"])
                .redis_connection_timeout(std::time::Duration::from_secs(5))
                .redis_response_timeout(std::time::Duration::from_secs(5))
                .build(),
        },
    )
    .await
}
