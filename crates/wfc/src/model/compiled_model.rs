use crate::model::rule_model::{AdjadencyRules, FrequencyHints};

#[derive(Debug)]
pub struct CompiledModel {
    pub adj_rules: AdjadencyRules,
    pub frequency_hints: FrequencyHints,
    pub num_patterns: usize,
    pub num_directions: usize,
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
