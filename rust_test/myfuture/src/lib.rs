use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

struct SharedState {
	completed: bool,
	waker: Option<Waker>
}

pub struct TimerFuture {
	sharedstate: Arc<Mutex<SharedState>>
}

impl Future for TimerFuture {
	type Output = ();
	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		let mut share_state = self.sharedstate.lock().unwrap();
		if share_state.completed {
			Poll::Ready(())
		} else {
			share_state.waker = Some(cx.waker().clone());
			Poll::Pending
		}
	}
}

impl TimerFuture {
	pub fn new(duration: Duration) -> Self {
		let share_state = Arc::new(Mutex::new(SharedState {
			completed: false,
			waker: None
		}));
		let t_share_state = share_state.clone();
		thread::spawn(move || {
			thread::sleep(duration);
			let mut s = t_share_state.lock().unwrap();
			s.completed = true;
			if let Some(w) = s.waker.take() {
				w.wake();
			}
		});
		TimerFuture { sharedstate: share_state }
	}
}