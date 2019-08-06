use std::any::Any;

pub(crate) trait RuntimeManager {
    fn unstable_shutdown();
    fn runtime_shutdown_callback();
}

pub(crate) trait FaultRecovery {
    fn panic_dispatcher(failure: Box<dyn Any + Send>);
}
