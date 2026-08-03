use std::fmt;

use crate::{
    CompiledModel, Dimensions,
    model::{direction::ALL_DIRECTIONS, pattern::Pattern, simple_bit_set::SimpleBitSet},
};

pub type AdjadencyRules = Vec<SimpleBitSet>;
pub struct RuleModel {
    pub patterns: Vec<Pattern>,
    pub adjadency_rules: AdjadencyRules,
    pub frequency_hints: FrequencyHints,
    pub num_directions: usize,
    pub pattern_dimensions: Dimensions<2>,
}

impl RuleModel {
    pub fn num_patterns(&self) -> usize {
        self.patterns.len()
    }

    pub fn compile(&self) -> Result<CompiledModel, RuleModelValidationErrors> {
        let structural = self.validate_shape();
        if !structural.is_empty() {
            return Err(RuleModelValidationErrors::new(structural));
        }

        let content = self.validate_content();
        if !content.is_empty() {
            return Err(RuleModelValidationErrors::new(content));
        }

        Ok(CompiledModel {
            adj_rules: self.adjadency_rules.clone(),
            frequency_hints: self.frequency_hints.clone(),
            num_patterns: self.num_patterns(),
            num_directions: self.num_directions,
        })
    }

    fn validate_shape(&self) -> Vec<RuleModelValidationError> {
        let num_patterns = self.num_patterns();
        let mut errors = Vec::new();

        if num_patterns == 0 {
            errors.push(RuleModelValidationError::NoPatterns);
        }

        if self.num_directions != ALL_DIRECTIONS.len() {
            errors.push(RuleModelValidationError::UnsupportedDirectionCount {
                found: self.num_directions,
                supported: ALL_DIRECTIONS.len(),
            });
        }

        let expected_rules = num_patterns * self.num_directions;
        if self.adjadency_rules.len() != expected_rules {
            errors.push(RuleModelValidationError::RuleTableShapeMismatch {
                expected: expected_rules,
                found: self.adjadency_rules.len(),
            });
        }

        if self.frequency_hints.weights.len() != num_patterns {
            errors.push(RuleModelValidationError::FrequencyCountMismatch {
                expected: num_patterns,
                found: self.frequency_hints.weights.len(),
            });
        }

        errors
    }

    fn validate_content(&self) -> Vec<RuleModelValidationError> {
        let num_patterns = self.num_patterns();
        let mut errors = Vec::new();

        for (pattern, weight) in self.frequency_hints.weights.iter().enumerate() {
            if *weight == 0 {
                errors.push(RuleModelValidationError::ZeroWeight { pattern });
            }
        }

        for pattern in 0..num_patterns {
            for direction in 0..self.num_directions {
                let rule = &self.adjadency_rules[pattern * self.num_directions + direction];

                if rule.count() == 0 {
                    continue;
                }

                for referenced in rule {
                    if referenced >= num_patterns {
                        errors.push(RuleModelValidationError::UnknownPatternReference {
                            pattern,
                            direction,
                            referenced,
                        });
                    }
                }
            }
        }

        errors
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleModelValidationError {
    NoPatterns,
    UnsupportedDirectionCount {
        found: usize,
        supported: usize,
    },
    RuleTableShapeMismatch {
        expected: usize,
        found: usize,
    },
    FrequencyCountMismatch {
        expected: usize,
        found: usize,
    },
    ZeroWeight {
        pattern: usize,
    },
    UnknownPatternReference {
        pattern: usize,
        direction: usize,
        referenced: usize,
    },
}

impl fmt::Display for RuleModelValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoPatterns => write!(f, "the model contains no patterns"),
            Self::UnsupportedDirectionCount { found, supported } => write!(
                f,
                "the model declares {found} directions, but the solver supports only {supported}"
            ),
            Self::RuleTableShapeMismatch { expected, found } => write!(
                f,
                "the rule table holds {found} entries, expected {expected} (patterns x directions)"
            ),
            Self::FrequencyCountMismatch { expected, found } => {
                write!(f, "the model holds {found} weights, expected {expected}")
            }
            Self::ZeroWeight { pattern } => {
                write!(f, "pattern {pattern} has a weight of zero")
            }
            Self::UnknownPatternReference {
                pattern,
                direction,
                referenced,
            } => write!(
                f,
                "pattern {pattern} allows unknown pattern {referenced} in direction {direction}"
            ),
        }
    }
}

impl std::error::Error for RuleModelValidationError {}

/// Every reason a [`RuleModel`] could not be compiled
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleModelValidationErrors(Vec<RuleModelValidationError>);

impl RuleModelValidationErrors {
    fn new(errors: Vec<RuleModelValidationError>) -> Self {
        assert!(
            !errors.is_empty(),
            "a validation failure must report at least one error"
        );
        Self(errors)
    }

    pub fn as_slice(&self) -> &[RuleModelValidationError] {
        &self.0
    }

    pub fn first(&self) -> &RuleModelValidationError {
        &self.0[0]
    }
}

impl fmt::Display for RuleModelValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "the rule model is invalid:")?;
        for error in &self.0 {
            writeln!(f, "  - {error}")?;
        }
        Ok(())
    }
}

impl std::error::Error for RuleModelValidationErrors {}
#[derive(PartialEq, Debug, Clone)]
pub struct FrequencyHints {
    pub weights: Vec<u32>,
    pub weighted_logs: Vec<f32>,
}

impl FrequencyHints {
    pub fn new(weights: Vec<u32>) -> Self {
        let weighted_logs = weights
            .iter()
            .map(|w| {
                let weight = *w as f32;
                weight * weight.log2()
            })
            .collect();
        Self {
            weights,
            weighted_logs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod compile {
        use super::*;

        fn valid_model() -> RuleModel {
            let num_directions = ALL_DIRECTIONS.len();
            let mut adjadency_rules = vec![SimpleBitSet::new(2); 2 * num_directions];
            for direction in 0..num_directions {
                adjadency_rules[direction].set(1);
                adjadency_rules[num_directions + direction].set(0);
            }
            RuleModel {
                patterns: vec![pattern(0), pattern(1)],
                adjadency_rules,
                frequency_hints: FrequencyHints::new(vec![1, 1]),
                num_directions,
                pattern_dimensions: Dimensions::new([1, 1]).expect("1x1 is non-empty"),
            }
        }

        fn pattern(sample: u32) -> Pattern {
            Pattern {
                samples: vec![sample],
                width: 1,
                height: 1,
            }
        }

        fn errors_of(model: &RuleModel) -> Vec<RuleModelValidationError> {
            model
                .compile()
                .expect_err("expected validation to fail")
                .as_slice()
                .to_vec()
        }

        #[test]
        fn compiles_a_valid_model() {
            let compiled = valid_model().compile().expect("the model is valid");

            assert_eq!(compiled.num_patterns, 2);
            assert_eq!(compiled.num_directions, ALL_DIRECTIONS.len());
        }

        #[test]
        fn rejects_a_model_with_no_patterns() {
            let mut model = valid_model();
            model.patterns.clear();

            assert!(errors_of(&model).contains(&RuleModelValidationError::NoPatterns));
        }

        #[test]
        fn rejects_an_unsupported_direction_count() {
            let mut model = valid_model();
            model.num_directions = 6;

            assert!(errors_of(&model).contains(
                &RuleModelValidationError::UnsupportedDirectionCount {
                    found: 6,
                    supported: 4,
                }
            ));
        }

        #[test]
        fn rejects_a_rule_table_of_the_wrong_length() {
            let mut model = valid_model();
            model.adjadency_rules.pop();

            assert!(errors_of(&model).contains(
                &RuleModelValidationError::RuleTableShapeMismatch {
                    expected: 8,
                    found: 7,
                }
            ));
        }

        #[test]
        fn rejects_a_weight_count_that_does_not_match_the_patterns() {
            let mut model = valid_model();
            model.frequency_hints = FrequencyHints::new(vec![1]);

            assert!(errors_of(&model).contains(
                &RuleModelValidationError::FrequencyCountMismatch {
                    expected: 2,
                    found: 1,
                }
            ));
        }

        #[test]
        fn rejects_a_zero_weight() {
            let mut model = valid_model();
            model.frequency_hints = FrequencyHints::new(vec![1, 0]);

            assert!(
                errors_of(&model).contains(&RuleModelValidationError::ZeroWeight { pattern: 1 })
            );
        }

        #[test]
        fn accepts_a_pattern_with_no_allowed_neighbours() {
            let mut model = valid_model();
            model.adjadency_rules[2].clear(1);

            assert!(model.compile().is_ok());
        }

        #[test]
        fn rejects_a_rule_referencing_an_unknown_pattern() {
            let mut model = valid_model();
            model.adjadency_rules[0].set(37);

            assert!(errors_of(&model).contains(
                &RuleModelValidationError::UnknownPatternReference {
                    pattern: 0,
                    direction: 0,
                    referenced: 37,
                }
            ));
        }

        #[test]
        fn reports_every_structural_error_at_once() {
            let mut model = valid_model();
            model.patterns.clear();

            let errors = errors_of(&model);

            assert_eq!(errors.len(), 3);
            assert!(errors.contains(&RuleModelValidationError::NoPatterns));
            assert!(
                errors.contains(&RuleModelValidationError::RuleTableShapeMismatch {
                    expected: 0,
                    found: 8,
                })
            );
            assert!(
                errors.contains(&RuleModelValidationError::FrequencyCountMismatch {
                    expected: 0,
                    found: 2,
                })
            );
        }

        #[test]
        fn does_not_run_content_checks_when_the_shape_is_wrong() {
            let mut model = valid_model();
            model.adjadency_rules.pop();
            model.frequency_hints = FrequencyHints::new(vec![1, 0]);

            let errors = errors_of(&model);

            assert!(!errors.contains(&RuleModelValidationError::ZeroWeight { pattern: 1 }));
        }
    }
}
