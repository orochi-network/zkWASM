use crate::proof_context::trace::state_trace_manager::StateTraceManager;
use crate::proof_context::trace::state_trace_tuple::StateTraceTuple;

pub struct ProofContext {
    state_trace_manager: StateTraceManager,
    time_stamp: u64,

    param_start: u64,
}

impl ProofContext {
    pub fn new(param_start: u64) -> Self {
        Self {
            state_trace_manager: StateTraceManager::new(),
            time_stamp: 0,
            param_start,
        }
    }

    pub fn get_time_stamp_then_increase(&mut self) -> u64 {
        let res = self.time_stamp.clone();
        self.time_stamp += 1;
        res
    }

    pub fn get_state_trace_manager(&self) -> &StateTraceManager {
        &self.state_trace_manager
    }

    pub(crate) fn add_state_trace_tuple(&mut self, state_trace_tuple: &StateTraceTuple) {
        self.state_trace_manager.add_state_trace_tuple(state_trace_tuple);
    }

    pub(crate) fn get_param_start(&self) -> u64 {
        self.param_start
    }
}