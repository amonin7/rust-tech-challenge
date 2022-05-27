#[derive(Debug)]
pub struct LoopInfo {
    pub start_op_index: usize,
    pub end_op_index: usize,
    pub iterations_num: u32,
}