#![cfg(not(miri))]

use anyhow::{anyhow, bail, Result};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use wasmtime::*;

fn async_store() -> Store<()> {
    Store::new(&Engine::new(Config::new().async_support(true)).unwrap(), ())
}

async fn run_smoke_test(store: &mut Store<()>, func: Func) {
    func.call_async(&mut *store, &[], &mut []).await.unwrap();
    func.call_async(&mut *store, &[], &mut []).await.unwrap();
}

async fn run_smoke_typed_test(store: &mut Store<()>, func: Func) {
    let func = func.typed::<(), ()>(&store).unwrap();
    func.call_async(&mut *store, ()).await.unwrap();
    func.call_async(&mut *store, ()).await.unwrap();
}

#[tokio::test]
async fn smoke() {
    let mut store = async_store();
    let func = Func::new_async(
        &mut store,
        FuncType::new(None, None),
        move |_caller, _params, _results| Box::new(async { Ok(()) }),
    );
    run_smoke_test(&mut store, func).await;
    run_smoke_typed_test(&mut store, func).await;

    let func = Func::wrap0_async(&mut store, move |_caller| Box::new(async { Ok(()) }));
    run_smoke_test(&mut store, func).await;
    run_smoke_typed_test(&mut store, func).await;
}

#[tokio::test]
async fn smoke_host_func() -> Result<()> {
    let mut store = async_store();
    let mut linker = Linker::new(store.engine());

    linker.func_new_async(
        "",
        "first",
        FuncType::new(None, None),
        move |_caller, _params, _results| Box::new(async { Ok(()) }),
    )?;

    linker.func_wrap0_async("", "second", move |_caller| Box::new(async { Ok(()) }))?;

    let func = linker
        .get(&mut store, "", "first")
        .unwrap()
        .into_func()
        .unwrap();
    run_smoke_test(&mut store, func).await;
    run_smoke_typed_test(&mut store, func).await;

    let func = linker
        .get(&mut store, "", "second")
        .unwrap()
        .into_func()
        .unwrap();
    run_smoke_test(&mut store, func).await;
    run_smoke_typed_test(&mut store, func).await;

    Ok(())
}

#[tokio::test]
async fn smoke_with_suspension() {
    let mut store = async_store();
    let func = Func::new_async(
        &mut store,
        FuncType::new(None, None),
        move |_caller, _params, _results| {
            Box::new(async {
                tokio::task::yield_now().await;
                Ok(())
            })
        },
    );
    run_smoke_test(&mut store, func).await;
    run_smoke_typed_test(&mut store, func).await;

    let func = Func::wrap0_async(&mut store, move |_caller| {
        Box::new(async {
            tokio::task::yield_now().await;
            Ok(())
        })
    });
    run_smoke_test(&mut store, func).await;
    run_smoke_typed_test(&mut store, func).await;
}

#[tokio::test]
async fn smoke_host_func_with_suspension() -> Result<()> {
    let mut store = async_store();
    let mut linker = Linker::new(store.engine());

    linker.func_new_async(
        "",
        "first",
        FuncType::new(None, None),
        move |_caller, _params, _results| {
            Box::new(async {
                tokio::task::yield_now().await;
                Ok(())
            })
        },
    )?;

    linker.func_wrap0_async("", "second", move |_caller| {
        Box::new(async {
            tokio::task::yield_now().await;
            Ok(())
        })
    })?;

    let func = linker
        .get(&mut store, "", "first")
        .unwrap()
        .into_func()
        .unwrap();
    run_smoke_test(&mut store, func).await;
    run_smoke_typed_test(&mut store, func).await;

    let func = linker
        .get(&mut store, "", "second")
        .unwrap()
        .into_func()
        .unwrap();
    run_smoke_test(&mut store, func).await;
    run_smoke_typed_test(&mut store, func).await;

    Ok(())
}

#[tokio::test]
async fn recursive_call() {
    let mut store = async_store();
    let async_wasm_func = Func::new_async(
        &mut store,
        FuncType::new(None, None),
        |_caller, _params, _results| {
            Box::new(async {
                tokio::task::yield_now().await;
                Ok(())
            })
        },
    );

    // Create an imported function which recursively invokes another wasm
    // function asynchronously, although this one is just our own host function
    // which suffices for this test.
    let func2 = Func::new_async(
        &mut store,
        FuncType::new(None, None),
        move |mut caller, _params, _results| {
            Box::new(async move {
                async_wasm_func
                    .call_async(&mut caller, &[], &mut [])
                    .await?;
                Ok(())
            })
        },
    );

    // Create an instance which calls an async import twice.
    let module = Module::new(
        store.engine(),
        "
            (module
                (import \"\" \"\" (func))
                (func (export \"\")
                    ;; call imported function which recursively does an async
                    ;; call
                    call 0
                    ;; do it again, and our various pointers all better align
                    call 0))
        ",
    )
    .unwrap();

    let instance = Instance::new_async(&mut store, &module, &[func2.into()])
        .await
        .unwrap();
    let func = instance.get_func(&mut store, "").unwrap();
    func.call_async(&mut store, &[], &mut []).await.unwrap();
}

#[tokio::test]
async fn suspend_while_suspending() {
    let mut store = async_store();

    // Create a synchronous function which calls our asynchronous function and
    // runs it locally. This shouldn't generally happen but we know everything
    // is synchronous in this test so it's fine for us to do this.
    //
    // The purpose of this test is intended to stress various cases in how
    // we manage pointers in ways that are not necessarily common but are still
    // possible in safe code.
    let async_thunk = Func::new_async(
        &mut store,
        FuncType::new(None, None),
        |_caller, _params, _results| Box::new(async { Ok(()) }),
    );
    let sync_call_async_thunk = Func::new(
        &mut store,
        FuncType::new(None, None),
        move |mut caller, _params, _results| {
            let mut future = Box::pin(async_thunk.call_async(&mut caller, &[], &mut []));
            let poll = future
                .as_mut()
                .poll(&mut Context::from_waker(&noop_waker()));
            assert!(poll.is_ready());
            Ok(())
        },
    );

    // A small async function that simply awaits once to pump the loops and
    // then finishes.
    let async_import = Func::new_async(
        &mut store,
        FuncType::new(None, None),
        move |_caller, _params, _results| {
            Box::new(async move {
                tokio::task::yield_now().await;
                Ok(())
            })
        },
    );

    let module = Module::new(
        store.engine(),
        "
            (module
                (import \"\" \"\" (func $sync_call_async_thunk))
                (import \"\" \"\" (func $async_import))
                (func (export \"\")
                    ;; Set some store-local state and pointers
                    call $sync_call_async_thunk
                    ;; .. and hopefully it's all still configured correctly
                    call $async_import))
        ",
    )
    .unwrap();
    let instance = Instance::new_async(
        &mut store,
        &module,
        &[sync_call_async_thunk.into(), async_import.into()],
    )
    .await
    .unwrap();
    let func = instance.get_func(&mut store, "").unwrap();
    func.call_async(&mut store, &[], &mut []).await.unwrap();
}

#[tokio::test]
async fn cancel_during_run() {
    let mut store = Store::new(&Engine::new(Config::new().async_support(true)).unwrap(), 0);

    let async_thunk = Func::new_async(
        &mut store,
        FuncType::new(None, None),
        move |mut caller, _params, _results| {
            assert_eq!(*caller.data(), 0);
            *caller.data_mut() = 1;
            let dtor = SetOnDrop(caller);
            Box::new(async move {
                // SetOnDrop is not destroyed when dropping the reference of it
                // here. Instead, it is moved into the future where it's forced
                // to live in and will be destroyed at the end of the future.
                let _ = &dtor;
                tokio::task::yield_now().await;
                Ok(())
            })
        },
    );
    // Shouldn't have called anything yet...
    assert_eq!(*store.data(), 0);

    // Create our future, but as per async conventions this still doesn't
    // actually do anything. No wasm or host function has been called yet.
    let future = Box::pin(async_thunk.call_async(&mut store, &[], &mut []));

    // Push the future forward one tick, which actually runs the host code in
    // our async func. Our future is designed to be pending once, however.
    let future = PollOnce::new(future).await;

    // Now that our future is running (on a separate, now-suspended fiber), drop
    // the future and that should deallocate all the Rust bits as well.
    drop(future);
    assert_eq!(*store.data(), 2);

    struct SetOnDrop<'a>(Caller<'a, usize>);

    impl Drop for SetOnDrop<'_> {
        fn drop(&mut self) {
            assert_eq!(*self.0.data(), 1);
            *self.0.data_mut() = 2;
        }
    }
}

#[tokio::test]
async fn iloop_with_fuel() {
    let engine = Engine::new(Config::new().async_support(true).consume_fuel(true)).unwrap();
    let mut store = Store::new(&engine, ());
    store.out_of_fuel_async_yield(1_000, 10);
    let module = Module::new(
        &engine,
        "
            (module
                (func (loop br 0))
                (start 0)
            )
        ",
    )
    .unwrap();
    let instance = Instance::new_async(&mut store, &module, &[]);

    // This should yield a bunch of times but eventually finish
    let (_, pending) = CountPending::new(Box::pin(instance)).await;
    assert!(pending > 100);
}

#[tokio::test]
async fn fuel_eventually_finishes() {
    let engine = Engine::new(Config::new().async_support(true).consume_fuel(true)).unwrap();
    let mut store = Store::new(&engine, ());
    store.out_of_fuel_async_yield(u64::max_value(), 10);
    let module = Module::new(
        &engine,
        "
            (module
                (func
                    (local i32)
                    i32.const 100
                    local.set 0
                    (loop
                        local.get 0
                        i32.const -1
                        i32.add
                        local.tee 0
                        br_if 0)
                )
                (start 0)
            )
        ",
    )
    .unwrap();
    let instance = Instance::new_async(&mut store, &module, &[]);
    instance.await.unwrap();
}

#[tokio::test]
async fn async_with_pooling_stacks() {
    let mut pool = PoolingAllocationConfig::default();
    pool.instance_count(1)
        .instance_memory_pages(1)
        .instance_table_elements(0);
    let mut config = Config::new();
    config.async_support(true);
    config.allocation_strategy(InstanceAllocationStrategy::Pooling(pool));
    config.dynamic_memory_guard_size(0);
    config.static_memory_guard_size(0);
    config.static_memory_maximum_size(65536);

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, ());
    let func = Func::new_async(
        &mut store,
        FuncType::new(None, None),
        move |_caller, _params, _results| Box::new(async { Ok(()) }),
    );

    run_smoke_test(&mut store, func).await;
    run_smoke_typed_test(&mut store, func).await;
}

#[tokio::test]
async fn async_host_func_with_pooling_stacks() -> Result<()> {
    let mut pooling = PoolingAllocationConfig::default();
    pooling
        .instance_count(1)
        .instance_memory_pages(1)
        .instance_table_elements(0);
    let mut config = Config::new();
    config.async_support(true);
    config.allocation_strategy(InstanceAllocationStrategy::Pooling(pooling));
    config.dynamic_memory_guard_size(0);
    config.static_memory_guard_size(0);
    config.static_memory_maximum_size(65536);

    let mut store = Store::new(&Engine::new(&config)?, ());
    let mut linker = Linker::new(store.engine());
    linker.func_new_async(
        "",
        "",
        FuncType::new(None, None),
        move |_caller, _params, _results| Box::new(async { Ok(()) }),
    )?;

    let func = linker.get(&mut store, "", "").unwrap().into_func().unwrap();
    run_smoke_test(&mut store, func).await;
    run_smoke_typed_test(&mut store, func).await;
    Ok(())
}

async fn execute_across_threads<F: Future + Send + 'static>(future: F) {
    let future = PollOnce::new(Box::pin(future)).await;

    tokio::task::spawn_blocking(move || future)
        .await
        .expect("shouldn't panic");
}

#[tokio::test]
async fn resume_separate_thread() {
    // This test will poll the following future on two threads. Simulating a
    // trap requires accessing TLS info, so that should be preserved correctly.
    execute_across_threads(async {
        let mut store = async_store();
        let module = Module::new(
            store.engine(),
            "
            (module
                (import \"\" \"\" (func))
                (start 0)
            )
            ",
        )
        .unwrap();
        let func = Func::wrap0_async(&mut store, |_| {
            Box::new(async {
                tokio::task::yield_now().await;
                Err::<(), _>(anyhow!("test"))
            })
        });
        let result = Instance::new_async(&mut store, &module, &[func.into()]).await;
        assert!(result.is_err());
    })
    .await;
}

#[tokio::test]
async fn resume_separate_thread2() {
    // This test will poll the following future on two threads. Catching a
    // signal requires looking up TLS information to determine whether it's a
    // trap to handle or not, so that must be preserved correctly across threads.
    execute_across_threads(async {
        let mut store = async_store();
        let module = Module::new(
            store.engine(),
            "
            (module
                (import \"\" \"\" (func))
                (func $start
                    call 0
                    unreachable)
                (start $start)
            )
            ",
        )
        .unwrap();
        let func = Func::wrap0_async(&mut store, |_| {
            Box::new(async {
                tokio::task::yield_now().await;
            })
        });
        let result = Instance::new_async(&mut store, &module, &[func.into()]).await;
        assert!(result.is_err());
    })
    .await;
}

#[tokio::test]
async fn resume_separate_thread3() {
    let _ = env_logger::try_init();

    // This test doesn't actually do anything with cross-thread polls, but
    // instead it deals with scheduling futures at "odd" times.
    //
    // First we'll set up a *synchronous* call which will initialize TLS info.
    // This call is simply to a host-defined function, but it still has the same
    // "enter into wasm" semantics since it's just calling a trampoline. In this
    // situation we'll set up the TLS info so it's in place while the body of
    // the function executes...
    let mut store = Store::new(&Engine::default(), None);
    let f = Func::wrap(&mut store, move |mut caller: Caller<'_, _>| -> Result<()> {
        // ... and the execution of this host-defined function (while the TLS
        // info is initialized), will set up a recursive call into wasm. This
        // recursive call will be done asynchronously so we can suspend it
        // halfway through.
        let f = async {
            let mut store = async_store();
            let module = Module::new(
                store.engine(),
                "
                    (module
                        (import \"\" \"\" (func))
                        (start 0)
                    )
                ",
            )
            .unwrap();
            let func = Func::wrap0_async(&mut store, |_| {
                Box::new(async {
                    tokio::task::yield_now().await;
                })
            });
            drop(Instance::new_async(&mut store, &module, &[func.into()]).await);
            unreachable!()
        };
        let mut future = Box::pin(f);
        let poll = future
            .as_mut()
            .poll(&mut Context::from_waker(&noop_waker()));
        assert!(poll.is_pending());

        // ... so at this point our call into wasm is suspended. The call into
        // wasm will have overwritten TLS info, and we sure hope that the
        // information is restored at this point. Note that we squirrel away the
        // future somewhere else to get dropped later. If we were to drop it
        // here then we would reenter the future's suspended stack to clean it
        // up, which would do more alterations of TLS information we're not
        // testing here.
        *caller.data_mut() = Some(future);

        // ... all in all this function will need access to the original TLS
        // information to raise the trap. This TLS information should be
        // restored even though the asynchronous execution is suspended.
        bail!("")
    });
    assert!(f.call(&mut store, &[], &mut []).is_err());
}

#[tokio::test]
async fn recursive_async() -> Result<()> {
    let _ = env_logger::try_init();
    let mut store = async_store();
    let m = Module::new(
        store.engine(),
        "(module
            (func (export \"overflow\") call 0)
            (func (export \"normal\"))
        )",
    )?;
    let i = Instance::new_async(&mut store, &m, &[]).await?;
    let overflow = i.get_typed_func::<(), ()>(&mut store, "overflow")?;
    let normal = i.get_typed_func::<(), ()>(&mut store, "normal")?;
    let f2 = Func::wrap0_async(&mut store, move |mut caller| {
        Box::new(async move {
            // recursive async calls shouldn't immediately stack overflow...
            normal.call_async(&mut caller, ()).await?;

            // ... but calls that actually stack overflow should indeed stack
            // overflow
            let err = overflow
                .call_async(&mut caller, ())
                .await
                .unwrap_err()
                .downcast::<Trap>()?;
            assert_eq!(err, Trap::StackOverflow);
            Ok(())
        })
    });
    f2.call_async(&mut store, &[], &mut []).await?;
    Ok(())
}

#[tokio::test]
async fn linker_module_command() -> Result<()> {
    let mut store = async_store();
    let mut linker = Linker::new(store.engine());
    let module1 = Module::new(
        store.engine(),
        r#"
            (module
                (global $g (mut i32) (i32.const 0))

                (func (export "_start"))

                (func (export "g") (result i32)
                    global.get $g
                    i32.const 1
                    global.set $g)
            )
        "#,
    )?;
    let module2 = Module::new(
        store.engine(),
        r#"
            (module
                (import "" "g" (func (result i32)))

                (func (export "get") (result i32)
                    call 0)
            )
        "#,
    )?;

    linker.module_async(&mut store, "", &module1).await?;
    let instance = linker.instantiate_async(&mut store, &module2).await?;
    let f = instance.get_typed_func::<(), i32>(&mut store, "get")?;
    assert_eq!(f.call_async(&mut store, ()).await?, 0);
    assert_eq!(f.call_async(&mut store, ()).await?, 0);

    Ok(())
}

#[tokio::test]
async fn linker_module_reactor() -> Result<()> {
    let mut store = async_store();
    let mut linker = Linker::new(store.engine());
    let module1 = Module::new(
        store.engine(),
        r#"
            (module
                (global $g (mut i32) (i32.const 0))

                (func (export "g") (result i32)
                    global.get $g
                    i32.const 1
                    global.set $g)
            )
        "#,
    )?;
    let module2 = Module::new(
        store.engine(),
        r#"
            (module
                (import "" "g" (func (result i32)))

                (func (export "get") (result i32)
                    call 0)
            )
        "#,
    )?;

    linker.module_async(&mut store, "", &module1).await?;
    let instance = linker.instantiate_async(&mut store, &module2).await?;
    let f = instance.get_typed_func::<(), i32>(&mut store, "get")?;
    assert_eq!(f.call_async(&mut store, ()).await?, 0);
    assert_eq!(f.call_async(&mut store, ()).await?, 1);

    Ok(())
}

pub struct CountPending<F> {
    future: F,
    yields: usize,
}

impl<F> CountPending<F> {
    pub fn new(future: F) -> CountPending<F> {
        CountPending { future, yields: 0 }
    }
}

impl<F> Future for CountPending<F>
where
    F: Future + Unpin,
{
    type Output = (F::Output, usize);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.future).poll(cx) {
            Poll::Pending => {
                self.yields += 1;
                Poll::Pending
            }
            Poll::Ready(e) => Poll::Ready((e, self.yields)),
        }
    }
}

pub struct PollOnce<F>(Option<F>);

impl<F> PollOnce<F> {
    pub fn new(future: F) -> PollOnce<F> {
        PollOnce(Some(future))
    }
}

impl<F> Future for PollOnce<F>
where
    F: Future + Unpin,
{
    type Output = F;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<F> {
        let mut future = self.0.take().unwrap();
        match Pin::new(&mut future).poll(cx) {
            Poll::Pending => Poll::Ready(future),
            Poll::Ready(_) => panic!("should not be ready"),
        }
    }
}

fn noop_waker() -> Waker {
    const VTABLE: RawWakerVTable =
        RawWakerVTable::new(|ptr| RawWaker::new(ptr, &VTABLE), |_| {}, |_| {}, |_| {});
    const RAW: RawWaker = RawWaker::new(0 as *const (), &VTABLE);
    unsafe { Waker::from_raw(RAW) }
}

#[tokio::test]
async fn non_stacky_async_activations() -> Result<()> {
    let mut config = Config::new();
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let mut store1: Store<Option<Pin<Box<dyn Future<Output = Result<()>> + Send>>>> =
        Store::new(&engine, None);
    let mut linker1 = Linker::new(&engine);

    let module1 = Module::new(
        &engine,
        r#"
            (module $m1
                (import "" "host_capture_stack" (func $host_capture_stack))
                (import "" "start_async_instance" (func $start_async_instance))
                (func $capture_stack (export "capture_stack")
                    call $host_capture_stack
                )
                (func $run_sync (export "run_sync")
                    call $start_async_instance
                )
            )
        "#,
    )?;

    let module2 = Module::new(
        &engine,
        r#"
            (module $m2
                (import "" "yield" (func $yield))

                (func $run_async (export "run_async")
                    call $yield
                )
            )
        "#,
    )?;

    let stacks = Arc::new(Mutex::new(vec![]));
    fn capture_stack(stacks: &Arc<Mutex<Vec<WasmBacktrace>>>, store: impl AsContext) {
        let mut stacks = stacks.lock().unwrap();
        stacks.push(wasmtime::WasmBacktrace::force_capture(store));
    }

    linker1.func_wrap0_async("", "host_capture_stack", {
        let stacks = stacks.clone();
        move |caller| {
            capture_stack(&stacks, &caller);
            Box::new(async { Ok(()) })
        }
    })?;

    linker1.func_wrap0_async("", "start_async_instance", {
        let stacks = stacks.clone();
        move |mut caller| {
            let stacks = stacks.clone();
            capture_stack(&stacks, &caller);

            let module2 = module2.clone();
            let mut store2 = Store::new(caller.engine(), ());
            let mut linker2 = Linker::new(caller.engine());
            linker2
                .func_wrap0_async("", "yield", {
                    let stacks = stacks.clone();
                    move |caller| {
                        let stacks = stacks.clone();
                        Box::new(async move {
                            capture_stack(&stacks, &caller);
                            tokio::task::yield_now().await;
                            capture_stack(&stacks, &caller);
                            Ok(())
                        })
                    }
                })
                .unwrap();

            Box::new(async move {
                let future = PollOnce::new(Box::pin({
                    let stacks = stacks.clone();
                    async move {
                        let instance2 = linker2.instantiate_async(&mut store2, &module2).await?;

                        instance2
                            .get_func(&mut store2, "run_async")
                            .unwrap()
                            .call_async(&mut store2, &[], &mut [])
                            .await?;

                        capture_stack(&stacks, &store2);
                        Ok(())
                    }
                }) as _)
                .await;
                capture_stack(&stacks, &caller);
                *caller.data_mut() = Some(future);
                Ok(())
            })
        }
    })?;

    let instance1 = linker1.instantiate_async(&mut store1, &module1).await?;
    instance1
        .get_typed_func::<(), ()>(&mut store1, "run_sync")?
        .call_async(&mut store1, ())
        .await?;
    let future = store1.data_mut().take().unwrap();
    future.await?;

    instance1
        .get_typed_func::<(), ()>(&mut store1, "capture_stack")?
        .call_async(&mut store1, ())
        .await?;

    let stacks = stacks.lock().unwrap();
    eprintln!("stacks = {stacks:#?}");

    assert_eq!(stacks.len(), 6);
    for (actual, expected) in stacks.iter().zip(vec![
        vec!["run_sync"],
        vec!["run_async"],
        vec!["run_sync"],
        vec!["run_async"],
        vec![],
        vec!["capture_stack"],
    ]) {
        eprintln!("expected = {expected:?}");
        eprintln!("actual = {actual:?}");
        assert_eq!(actual.frames().len(), expected.len());
        for (actual, expected) in actual.frames().iter().zip(expected) {
            assert_eq!(actual.func_name(), Some(expected));
        }
    }

    Ok(())
}
