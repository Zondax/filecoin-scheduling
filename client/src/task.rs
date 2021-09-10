use scheduler::ResourceAlloc;
pub trait TaskFunc {
    type Output;
    type Error;

    fn init(&mut self, _: Option<&ResourceAlloc>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn end(&mut self, _: Option<&ResourceAlloc>) -> Result<Self::Output, Self::Error>;
    fn task(&mut self, alloc: Option<&ResourceAlloc>) -> Result<TaskResult, Self::Error>;
}

/// Helper type that indicates if a task should be executed again
#[derive(PartialEq, Eq)]
pub enum TaskResult {
    Continue,
    Done,
}

impl TaskResult {
    pub fn is_continue(&self) -> bool {
        matches!(self, Self::Continue)
    }
}
