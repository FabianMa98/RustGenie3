use std::collections::HashMap;

struct GeneExpressionMatrix {
    data: Vec<f64>,
    genes: Vec<String>,
    samples: Vec<String>,
    rows: usize,
    cols: usize,
}

impl GeneExpressionMatrix {
    /// Creeates new gene expression matrix with given genes and samples
    fn new(genes: Vec<String>, samples: Vec<String>) -> Self {
        let rows = genes.len();
        let cols = samples.len();

        GeneExpressionMatrix {
            data: vec![0.0; row * cols],
            genes,
            samples,
            rows,
            cols,
        }
    }

    /// Getter
    fn get(&self, gene: &str, sammple: &str) -> Option<f64> {
        let row = self.genes.iter().position(|g| g == gene)?;
        let col = self.samples.iter().position(|s| s == sample)?;

        Some(self.data[row * sef.cols + col])
    }

    /// Setter
    fn set(&mut self, gene: &str, sample: &str, value: f64) -> Result<(), String> {
        let row = self.genes.iter().position(|g| g == gene).ok_or("Gene was not found")?;
        let col = self.samples.iter().position(|s| s == sample).ok_or("Sample was not found")?;
        self.data[row * self.cols + col] = value;
        Ok(())
    }

    /// Norm matrix
    fn normalize_by_gene(&mut self) {
        for row in 0..self.rows {
            let row_start = row * self.cols;
            let row_end = row_start + self.cols;
            let gene_values = &mut self.data[row_start..row_end];
            let mean = gene_values.iter().copied().sum::<f64>() / gene_values.len() as f64;
            let std_dev = (gene_values.iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>()
                / gene_values.len() as f64)
                .sqrt();

            for value in gene_values {
                *value = (*value - mean) / std_dev;
            }
        }
    }

    // Getter for Gene Expression
    fn get_gene_expression(&self, gene: &str) -> Option<Vec<f64>> {
        let row = self.genes.iter().position(|g| g == gene)?;
        Some(self.data[row * self.cols..(row + 1) * self.cols].to_vec())
    }

    // Getter for Smaple Expression
    fn get_sample_expression(&self, sample: &str) -> Option<Vec<f64>> {
        
    }

}