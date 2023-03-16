use std::{future::Future, pin::Pin};

use pin_project::pin_project;

#[pin_project]
pub struct MeasurableFuture<Fut: Future> {
    #[pin]
    inner_future: Fut,
    started_at: Option<std::time::Instant>,
}

impl<Fut: Future> MeasurableFuture<Fut> {
    pub fn new(f: Fut) -> Self {
        MeasurableFuture { inner_future: f, started_at: None }
    }
}

impl<Fut: Future> Future for MeasurableFuture<Fut> {
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let projected_self = self.project();

        let started_at = projected_self.started_at.get_or_insert_with(std::time::Instant::now);

        if let std::task::Poll::Ready(res) = projected_self.inner_future.poll(cx) {
            println!(
                "{}ns elapsed for measured future",
                (std::time::Instant::now() - *started_at).as_nanos()
            );

            return std::task::Poll::Ready(res);
        }

        std::task::Poll::Pending
    }
}