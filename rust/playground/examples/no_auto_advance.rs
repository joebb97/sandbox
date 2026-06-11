fn main() {
    // let m: Arc<Mutex<Option<NetworkAction>>> = Arc::new(Mutex::new(None));
    // let mc = m.clone();
    // let mut h = harness.tanetworkd_handle;
    // let now = Instant::now();
    // tokio::spawn(async move {
    //     let a = h.action_recv().await;
    //     *mc.lock().expect("mutex poisoned") = Some(a);
    // });

    // tokio::time::advance(Duration::from_millis(50)).await;

    // while !m.lock().expect("mutex poisoned").is_some() {
    //     tokio::task::yield_now().await;
    // }

    // assert_eq!(
    //     *m.lock()
    //         .expect("mutex poisoned")
    //         .as_mut()
    //         .expect("checked some above"),
    //     NetworkAction::SetConfiguration(config_v2.config)
    // );

    // tracing::warn!("{:?}", now.elapsed());
    //
    // --------- This will hang forever if a.await gets stuck
    //   let now = Instant::now();
    //let a = tokio::task::spawn_blocking(move || harness.tanetworkd_handle.action_recv_blocking());

    // tokio::time::advance(Duration::from_millis(50)).await;
    // let a = a.await.unwrap();

    // assert_eq!(a, NetworkAction::SetConfiguration(config_v2.config));

    // tracing::warn!("{:?}", now.elapsed());
    // --------- This always sleeps
    // fn with_sys_timeout_panic(self, d: Duration) -> impl Future<Output = Self::Output> + Send
    // where
    //     Self: Send, {
    //     async move {
    //         tokio::select! {
    //             res = self => res,
    //             _ = task::spawn_blocking(move || std::thread::sleep(d)) => {
    //                 panic!("Future did not complete within {d:?}");
    //             }
    //         }
    //     }
    //
    // -------- This works but spawns a heavy handed os-thread
    //
    // pub async fn wall_time_timeout<F, T>(duration: Duration, fut: F) -> Result<T, ()>
    // where
    //     F: Future<Output = T> + Send + 'static,
    //     T: Send + 'static, {
    //     let (tx, rx) = oneshot::channel::<()>();
    //     std::thread::spawn(move || {
    //         std::thread::sleep(duration);
    //         let _ = tx.send(());
    //     });
    //     tokio::select! {
    //         result = fut => Ok(result),
    //         _ = rx => Err(()), // timed out
    //     }
    // }
    // ------- Just right
    // tokio::time::advance(Duration::from_millis(49)).await;
    // let a = tokio::task::spawn_blocking(move || harness.tanetworkd_handle.action_recv_blocking());
    // let wall_time = std::time::Instant::now();
    // let a = loop {
    //     if a.is_finished() {
    //         break a.await.unwrap();
    //     }
    //     if wall_time.elapsed() > Duration::from_secs(1) {
    //         panic!("timed out after 1s wall time");
    //     }
    //     tokio::task::yield_now().await;
    // };
}
