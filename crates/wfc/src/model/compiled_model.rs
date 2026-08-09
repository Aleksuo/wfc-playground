use crate::model::rule_model::{AdjadencyRules, FrequencyHints};

#[derive(Debug)]
pub struct CompiledModel {
    adj_rules: AdjadencyRules,
    frequency_hints: FrequencyHints,
    num_patterns: usize,
    num_directions: usize,
}

impl CompiledModel {
    pub(crate) fn new(
        adj_rules: AdjadencyRules,
        frequency_hints: FrequencyHints,
        num_directions: usize,
    ) -> Self {
        let num_patterns = frequency_hints.weights.len();
        Self {
            adj_rules,
            frequency_hints,
            num_patterns,
            num_directions,
        }
    }

    pub(crate) fn adj_rules(&self) -> &AdjadencyRules {
        &self.adj_rules
    }

    pub(crate) fn frequency_hints(&self) -> &FrequencyHints {
        &self.frequency_hints
    }

    pub(crate) fn num_patterns(&self) -> usize {
        self.num_patterns
    }

    pub(crate) fn num_directions(&self) -> usize {
        self.num_directions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const fn assert_send_sync<T: Send + Sync>() {}
    #[test]
    fn is_send_and_sync() {
        assert_send_sync::<CompiledModel>();
    }
}
