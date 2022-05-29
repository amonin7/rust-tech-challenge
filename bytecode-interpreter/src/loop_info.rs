
/// Structure represents all meta-info about loop in the bytecode
#[derive(Debug)]
pub struct LoopInfo {
    /// The index of the first operation in the loop
    pub start_op_index: usize,
    /// The index of the last operation in the loop
    pub end_op_index: usize,
    /// The number of operations in the loop
    pub iterations_num: u32,
    /// This value represents, whether current op is in the loop or outside of it
    pub has_loop: bool,
}