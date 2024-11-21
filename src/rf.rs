use std::time::Instant;
use std::thread;

use crate::{matrix, GeneExpressionMatrix};

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
            let zero_matrix: Vec<Vec<f32>> = self.create_zero_matrix_by_shape(); // This will be our final VIM
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

    fn filter_matrix_by_col(&self, exclusion_idx: Vec<i32>) -> Vec<Vec<f64>> {
        let mut new_matrix: Vec<Vec<f64>> = self.expression_matrix.data.clone();

        let return_matrix = new_matrix
        .iter()
        .map(|row| {
            row.iter() // put into iterator
                .enumerate() // same to python enumerate, generates (index, value) tuple
                .filter(|(col_idx, _)| !exclusion_idx.contains(col_idx))
                .map(|(_, value)| *value)
                .collect()
        })
        .collect();

        return_matrix

    }

    fn filter_matrix_by_val(&self, exclusion_val: Vec<&str>) -> Vec<Vec<f64>> {
        // Cloning expression_matrix data might actually slow this down by a lot
        let new_matrix = self.expression_matrix.data.clone();

        let return_matrix = new_matrix
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter(|_, col_val| !exclusion_val.contains(col_val))
                    .map(|_. value| *value)
                    .collect()
            })
            .collect();

        return_matrix
    }

    fn get_single_target_expression_value(&self, target: &str) {
        let target = self.expression_matrix.get_sample_expression(target);
        let norm_target = target.normalize_by_gene();

        return target
    }
}


/// Compute RandomForestRegressor for single target gene
impl<'a> GENIE3_single<'a> {
    pub fn new(
        expression_matrix: GeneExpressionMatrix<'a>,
        regulators: Vec<&'a str>,
        tree_method: String,
        k_number: String,
        n_trees: i32,
        n_threads: i32,
        target_gene: &'a str,
    ) -> Self {
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
        // Time Run
        let now = Instant::now();
        let ngenes = self.expression_matrix.rows();
        let output = 
        
        let mut target_gene_expression = self.expression_matrix.get_gene_expression(target_gene);
        let normalized_target_gene_expression = self.normalize_single_vec(target_gene_expression);

        if 
        let elapsed = now.elapsed();
    }

    fn normalize_single_vec(&self, target_vec: Vec<f32>) -> Vec<f32> {
        let magnitude = target_vec.iter().map(|&x| x*x).sum::<f32>().sqrt();
        if magnitude == 0.0 {
            target_vec.to_vec()
        } else {
            target_vec.iter().map(|&x| x / magnitude).collect()
        }
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

fn genie3_single(
    expr_data: &Array2<f64>,
    output_idx: usize,
    input_idx: &mut Vec<usize>,
    tree_method: &str,
    k: Option<usize>,
    ntrees: usize,
) -> Vec<f64> {
    let ngenes = expr_data.shape()[1];
    let output = expr_data.column(output_idx).to_owned();

    let normalized_output = normalize(&output);

    if let Some(pos) = input_idx.iter().position(|&x| x == output_idx) {
        input_idx.remove(pos);
    }

    // Extract candidate regulator data
    let expr_data_input = expr_data.select(Axis(1), &input_idx);

    // Determine max_features based on parameter K
    let max_features = match k {
        Some(k_val) if k_val < input_idx.len() => k_val,
        _ => "auto".to_string(), // Replace "auto" logic as per library
    };

    // Define the tree-based estimator
    let model: Box<dyn Regressor<Array2<f64>>> = match tree_method {
        "RF" => Box::new(
            RandomForestRegressor::fit(
                &expr_data_input,
                &normalized_output,
                RandomForestRegressorParameters::default()
                    .with_n_trees(ntrees)
                    .with_max_features(max_features),
            )
            .unwrap(),
        ),
        "ET" => Box::new(
            ExtraTreesRegressor::fit(
                &expr_data_input,
                &normalized_output,
                ExtraTreesRegressorParameters::default()
                    .with_n_trees(ntrees)
                    .with_max_features(max_features),
            )
            .unwrap(),
        ),
        _ => panic!("Invalid tree method! Use 'RF' or 'ET'."),
    };

    // Compute feature importances
    let feature_importances = compute_feature_importances(&*model);
    let mut vi = vec![0.0; ngenes];
    for (i, &importance) in input_idx.iter().zip(feature_importances.iter()) {
        vi[i] = importance;
    }

    vi
}