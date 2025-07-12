use std::{error::Error, fmt::Debug};

pub type ProcessResult<T> = Result<T, ProcessError>;

#[derive(Debug)]
pub struct ProcessError(Vec<(&'static str, Box<dyn Error>)>);

pub trait IntoProcessResult<T> {
    fn into_process_result(self, node: &'static str) -> ProcessResult<T>;
}

impl<T> IntoProcessResult<T> for T {
    fn into_process_result(self, _node: &'static str) -> ProcessResult<T> {
        Ok(self)
    }
}

impl<T, E: Error + 'static> IntoProcessResult<T> for Result<T, E> {
    fn into_process_result(self, node: &'static str) -> ProcessResult<T> {
        match self {
            Err(e) => Err(ProcessError(vec![(node, Box::new(e))])),
            Ok(v) => Ok(v),
        }
    }
}
