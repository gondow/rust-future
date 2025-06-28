#![feature(coroutines, coroutine_trait)]
#![feature(stmt_expr_attributes)]

use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;
use std::task::Poll;
use std::thread::sleep;
use std::time::Duration;
struct SimpleFuture {
    state: u8,
    pinned: Pin<Box<dyn Coroutine<Yield = u8, Return = u8>>>
}
type SimpleOutput = &'static str;
impl SimpleFuture {
    fn new() -> Self {
       let coro = #[coroutine] || {
           println!("Start");
           yield 1;
           println!("Middle");
           yield 2;
           println!("End");
           return 3;
        };
        Self { state: 0, pinned: Box::pin(coro), }
    }
    fn poll(mut self: Pin<&mut Self>) -> Poll<SimpleOutput> {
        match self.pinned.as_mut().resume(()) {
            CoroutineState::Yielded(val) => {
                println!("Yielded: {}->{}", self.state, val);
                self.state = val;
                Poll::Pending
            }
            CoroutineState::Complete(val) => {
                println!("Complete: {}->{}", self.state, val);
                self.state = val;
                Poll::Ready("Done")
            }
        }
    }
}

fn main() {
    let mut fut = SimpleFuture::new();
    let mut pinned = unsafe { Pin::new_unchecked(&mut fut) };

    loop {
        println!("loop...");
        match pinned.as_mut().poll() {
            Poll::Ready(val) => {
                println!("Coroutine returned: {}", val);
                break;
            }
            Poll::Pending => {
                println!("Coroutine yielded");
            }
        }
        sleep(Duration::from_secs(2));
    }
}

