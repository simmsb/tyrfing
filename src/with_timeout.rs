use embassy_time::Duration;

use core::future::Future;

pub(crate) struct WithTimeout<F> {
    pub(crate) timer: Option<embassy_time::Timer>,
    pub(crate) fut: F,
}

impl<F: Future> Future for WithTimeout<F> {
    type Output = Result<F::Output, ()>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        let a = unsafe {
            this.timer
                .as_mut()
                .map(|t| core::pin::Pin::new_unchecked(t))
        };
        let b = unsafe { core::pin::Pin::new_unchecked(&mut this.fut) };

        if let core::task::Poll::Ready(x) = b.poll(cx) {
            return core::task::Poll::Ready(Ok(x));
        }
        if let Some(core::task::Poll::Ready(_)) = a.map(|f| f.poll(cx)) {
            return core::task::Poll::Ready(Err(()));
        }
        core::task::Poll::Pending
    }
}

// this takes an option so that we don't have two codepaths for waiting with a delay, and waiting without

pub fn with_timeout<F: Future>(
    timeout: Option<Duration>,
    fut: F,
) -> impl Future<Output = Result<F::Output, ()>> {
    let timer = timeout.map(embassy_time::Timer::after);
    WithTimeout { timer, fut }
}
