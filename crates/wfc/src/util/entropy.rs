pub fn calculate_shannon_entropy(weight_sum: f32, weighted_log_sum: f32) -> f32 {
    weight_sum.log2() - (weighted_log_sum / weight_sum)
}
