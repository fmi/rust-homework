// Warning: the test uses the `futures` library.
// This is available only to the testing code (it is a dev-dependency),
// it is not available to the solution code.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use futures::{SinkExt, StreamExt};
use solution::*;

#[test]
fn test_futures_join() {
    run_with_timeout(1000, || async {
        let resource = Arc::new(MyMutex::new(vec![]));
        let (mut sender_3to1, mut recv_3to1) = futures::channel::mpsc::channel(1);

        let fut1 = {
            // A future that locks the resource, then holds the lock across an
            // await point (awaiting the next message from the channel).
            //
            // With an async-aware mutex this is not a problem - while `fut1` is blocked,
            // `fut2` and `fut3` can still run. So eventually `fut3` will send the message
            // and unblock `fut1`.
            let resource = Arc::clone(&resource);
            async move {
                let mut lock = resource.lock().await;
                let () = recv_3to1.next().await.unwrap();
                lock.push("one".to_string());
            }
        };

        let fut2 = {
            let resource = Arc::clone(&resource);
            async move {
                let mut lock = resource.lock().await;
                lock.push("two".to_string());
            }
        };

        let fut3 = {
            let resource = Arc::clone(&resource);
            async move {
                sender_3to1.send(()).await.unwrap();
                let mut lock = resource.lock().await;
                lock.push("three".to_string());
            }
        };

        // `join` polls the futures in order.
        //
        // Also `join` provides a single `Waker`, which just wakes the `Join3` future,
        // which every time polls each of the inner futures in order.
        // So using any waker will "wake" all three futures.
        futures::future::join3(fut1, fut2, fut3).await;

        assert_eq!(&*resource.lock().await, &["one", "two", "three"]);
    });
}

#[test]
fn test_futures_unordered() {
    run_with_timeout(1000, || async {
        let resource = Arc::new(MyMutex::new(vec![]));
        let (mut sender_3to1, mut recv_3to1) = futures::channel::mpsc::channel(1);

        let fut1 = pin_box({
            let resource = Arc::clone(&resource);
            async move {
                let mut lock = resource.lock().await;
                let () = recv_3to1.next().await.unwrap();
                lock.push("one".to_string());
            }
        });

        let fut2 = pin_box({
            let resource = Arc::clone(&resource);
            async move {
                let mut lock = resource.lock().await;
                lock.push("two".to_string());
            }
        });

        let fut3 = pin_box({
            let resource = Arc::clone(&resource);
            async move {
                sender_3to1.send(()).await.unwrap();
                let mut lock = resource.lock().await;
                lock.push("three".to_string());
            }
        });

        // Same example, but uses `FuturesUnordered` instead of `join`.
        //
        // `FuturesUnordered` doesn't guarantee any ordering.
        // Also it is more optimized for a large number of futures and will provide a separate
        // `Waker` for each of the inner futures.
        // So we can test that the correct wakers are being used.
        let mut unordered = futures::stream::FuturesUnordered::from_iter([fut1, fut2, fut3]);
        while let Some(_) = unordered.next().await {}

        let mut final_resource = resource.lock().await.clone();
        final_resource.sort();
        assert_eq!(final_resource, &["one", "three", "two"]);
    });
}

fn run_with_timeout<F, R>(timeout_millis: u64, test_fn: F)
where
    F: FnOnce() -> R + Send + std::panic::UnwindSafe + 'static,
    R: Future<Output = ()> + 'static,
{
    use futures::task::LocalSpawn;
    use std::panic::catch_unwind;
    use std::sync::mpsc;

    let (sender, receiver) = mpsc::sync_channel(1);

    std::thread::spawn(move || {
        let result = catch_unwind(move || {
            let mut runtime = futures::executor::LocalPool::new();

            let test_future = Box::new(test_fn());
            runtime.spawner().spawn_local_obj(test_future.into()).unwrap();

            runtime.run();
        });

        let _ = sender.send(result);
    });

    let timeout = std::time::Duration::from_millis(timeout_millis);
    match receiver.recv_timeout(timeout) {
        Ok(Ok(())) => {}
        Ok(Err(any)) => panic!("test panicked: {}", any.downcast::<&str>().unwrap()),
        Err(mpsc::RecvTimeoutError::Timeout) => panic!("test timed out"),
        Err(mpsc::RecvTimeoutError::Disconnected) => unreachable!(),
    }
}

fn pin_box<F>(fut: F) -> Pin<Box<dyn Future<Output = ()>>>
where
    F: Future<Output = ()> + 'static,
{
    Box::into_pin(Box::new(fut) as Box<dyn Future<Output = ()>>)
}
