#[derive(Debug)]
pub enum ProcessResult {
    StartLoop,
    EndLoop,
    Return,
    Continue,
}