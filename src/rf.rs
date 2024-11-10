use randomforest::criterion::Mse;
use randomforest::RandomForestRegressorOptions;
use randomforest::table::TableBuilder;
use std::time::Instant;
use matrix::GeneExpressionMatrix

/// In our case gene names are mandatory for this to ensure proper
/// running
pub struct GENIE3 {
    expression_matrix: GeneExpressionMatrix,
    regulators: Vec<String>,
    tree_method: String,
    k_number: String, 
    n_tress: i32,
    n_threads: i32,
}

impl GENIE3 {
    pub fn new(expression_matrix: GeneExpressionMatrix,
                regulators: Vec<String>,
                tree_method: String,
                k_number: String,
                n_trees: i32,
                n_threads: i32) -> Self {
                    GENIE3 {
                        expression_matrix,
                        regulators,
                        tree_method,
                        k_number,
                        n_trees,
                        n_threads
                    }
                }

    pub fn run(self) -> Vec<Vec<f32>>

}