use std::future::Future;
pub(crate) trait InstrumentedFuture: Future {
    fn id(&self) -> Option<tracing::Id>;
}
impl<F: Future> InstrumentedFuture for tracing::instrument::Instrumented<F> {
    fn id(&self) -> Option<tracing::Id> {
        panic!("STUB: not implemented");
    }
}
