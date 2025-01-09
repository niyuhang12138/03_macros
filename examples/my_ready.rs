use anyhow::Result;
use std::{future::Future, task::Poll};

#[tokio::main]
async fn main() -> Result<()> {
    let fut = MyFut::new(42);
    println!("fut: {fut:?}");
    Ok(())
}

#[derive(Debug)]
struct MyFut {
    polled: bool,
    v: usize,
}

impl MyFut {
    fn new(v: usize) -> Self {
        Self { polled: false, v }
    }
}

impl Future for MyFut {
    type Output = usize;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.v)
        } else {
            self.polled = true;
            // wake up the waker
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[macro_export]
macro_rules! my_ready {
    ($expr:expr) => {
        match $expr {
            std::task::Poll::Ready(val) => val,
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
