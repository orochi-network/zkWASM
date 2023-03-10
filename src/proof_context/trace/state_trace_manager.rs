use crate::proof_context::trace::state_trace_tuple::StateTraceTuple;

pub struct StateTraceManager {
    trace_vector: Vec<StateTraceTuple>,
}

impl StateTraceManager {
    pub fn new() -> Self {
        Self {
            trace_vector: vec![],
        }
    }

    pub(crate) fn add_state_trace_tuple(&mut self, state_trace_tuple: &StateTraceTuple) {
        self.trace_vector.push(state_trace_tuple.clone());
    }

    pub fn size(&self) -> usize {
        self.trace_vector.len()
    }

    pub fn get_state_trace_tupe(&self, index: usize) -> &StateTraceTuple {
        &self.trace_vector[index]
    }
}