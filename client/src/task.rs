use scheduler::{ResourceAlloc, TaskResult};
pub trait TaskFunc {
    type Output;
    type Error;

    fn init(&mut self, _: Option<&ResourceAlloc>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn end(&mut self, _: Option<&ResourceAlloc>) -> Result<Self::Output, Self::Error>;
    fn task(&mut self, alloc: Option<&ResourceAlloc>) -> Result<TaskResult, Self::Error>;
}
