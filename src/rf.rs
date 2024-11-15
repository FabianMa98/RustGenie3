use std::time::Instant;
use std::thread;

use crate::GeneExpressionMatrix;

/// In our case gene names are mandatory for this to ensure proper
/// running
/// TODO: Implement arg checking
pub struct GENIE3<'a> {
    expression_matrix: GeneExpressionMatrix<'a>,
    regulators: Vec<&'a str>,
    tree_method: String,
    k_number: String, 
    n_trees: i32,
    n_threads: i32,
}

impl<'a> GENIE3<'a> {
    pub fn new(expression_matrix: GeneExpressionMatrix<'a>,
                regulators: Vec<&'a str>,
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

    pub fn run(&self) -> Vec<Vec<f32>> {
        let now = Instant::now();
        let ngenes = self.expression_matrix.cols();
        if self.regulators == vec!["all"] {
            let input_idx: Vec<usize> = (0..ngenes).collect();

            println!("{:?}", input_idx);
        } else {
            let input_idx: Vec<usize> = gene_regulators_intersection(self.regulators.clone(), self.expression_matrix.genes.clone());
            let zero_matrix: Vec<Vec<f32>> = self.create_zero_matrix_by_shape();
            println!("{:?}", zero_matrix);
            println!("{:?}", input_idx);
        }
        let elapsed_time = now.elapsed();
        let shut_up_compiler: Vec<Vec<f32>> = vec![vec![1.000,2.000,3.000]];

        shut_up_compiler 
    }


    /// Allocate zero based variable importance matrix
    fn create_zero_matrix_by_shape(&self) -> Vec<Vec<f32>> {
        let zero_matrix = vec![vec![0f32; self.expression_matrix.cols]; self.expression_matrix.rows];

        zero_matrix
    }

}

pub fn gene_regulators_intersection(regulators: Vec<&str>, genes: Vec<&str>) -> Vec<usize> {
    genes
    .iter()
    .enumerate()
    .filter(|(_, gene)| regulators.contains(gene))
    .map(|(i, _)| i)
    .collect()
}