use log::{Level, error, info};
use std::{
    fmt::Debug,
    future::Future,
    panic::{RefUnwindSafe, UnwindSafe},
    pin::Pin,
    time::Instant,
};

//normal version for sync functions
pub fn log_wrap<F, Args, Ret>(
    func_name: &'static str,
    level: Level,
    func: F,
) -> impl Fn(Args) -> Ret
where
    F: Fn(Args) -> Ret + RefUnwindSafe,
    Args: Debug + UnwindSafe,
    Ret: Debug,
{
    move |args| {
        let start = Instant::now();
        if level <= Level::Info {
            info!("Calling {} with args: {:?}", func_name, args);
        }

        let result = std::panic::catch_unwind(|| func(args));

        match result {
            Ok(ret) => {
                if level <= Level::Info {
                    info!(
                        "{} returned {:?} in {}ms",
                        func_name,
                        ret,
                        start.elapsed().as_millis()
                    );
                }
                ret
            }
            Err(e) => {
                error!("{} panicked: {:?}", func_name, e);
                panic!("Function {} panicked", func_name);
            }
        }
    }
}

// Async version with boxed future
pub fn log_wrap_async<Fut, F, Args, Ret>(
    func_name: &'static str,
    level: Level,
    func: F,
) -> impl Fn(Args) -> Pin<Box<dyn Future<Output = Ret> + Send>>
where
    F: Fn(Args) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Ret> + Send + 'static,
    Args: Debug + Send + 'static,
    Ret: Debug + Send + 'static,
{
    move |args| {
        let start = Instant::now();
        if level <= Level::Info {
            info!("Calling {} with args: {:?}", func_name, args);
        }

        let fut = func(args);
        Box::pin(async move {
            let result = fut.await;
            if level <= Level::Info {
                info!(
                    "{} returned {:?} in {}ms",
                    func_name,
                    result,
                    start.elapsed().as_millis()
                );
            }
            result
        })
    }
}
