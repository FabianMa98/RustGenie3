pub struct GeneExpressionMatrix {
    data: Vec<Vec<f64>>,       // Changed to Vec<Vec<f64>> for double vector representation
    genes: Vec<String>,
    samples: Vec<String>,
    rows: usize,
    cols: usize,
}

impl GeneExpressionMatrix {
    /// Creates a new gene expression matrix with the given genes and samples
    pub fn new(genes: Vec<String>, samples: Vec<String>) -> Self {
        let rows = genes.len();
        let cols = samples.len();

        // Initialize a 2D vector with zeros for each gene-sample pair
        let data = vec![vec![0.0; cols]; rows];

        GeneExpressionMatrix {
            data,
            genes,
            samples,
            rows,
            cols,
        }
    }

    /// Getter for a specific gene and sample
    pub fn get(&self, gene: &str, sample: &str) -> Option<f64> {
        let row = self.genes.iter().position(|g| g == gene)?;
        let col = self.samples.iter().position(|s| s == sample)?;

        Some(self.data[row][col])
    }

    /// Setter for a specific gene and sample
    pub fn set(&mut self, gene: &str, sample: &str, value: f64) -> Result<(), String> {
        let row = self.genes.iter().position(|g| g == gene).ok_or("Gene was not found")?;
        let col = self.samples.iter().position(|s| s == sample).ok_or("Sample was not found")?;
        self.data[row][col] = value;
        Ok(())
    }

    /// Normalize each gene's expression values (rows) across all samples
    pub fn normalize_by_gene(&mut self) {
        for row in &mut self.data {
            let mean = row.iter().copied().sum::<f64>() / row.len() as f64;
            let std_dev = (row.iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>()
                / row.len() as f64)
                .sqrt();

            for value in row.iter_mut() {
                *value = (*value - mean) / std_dev;
            }
        }
    }

    /// Getter for all expression values of a specific gene (row)
    pub fn get_gene_expression(&self, gene: &str) -> Option<Vec<f64>> {
        let row = self.genes.iter().position(|g| g == gene)?;
        Some(self.data[row].clone())
    }

    /// Getter for all expression values of a specific sample (column)
    pub fn get_sample_expression(&self, sample: &str) -> Option<Vec<f64>> {
        let col = self.samples.iter().position(|s| s == sample)?;
        Some(self.data.iter().map(|row| row[col]).collect())
    }
    
    // Getter for number of rows (all genes)
    pub fn cols(&self9) -> usize {
        self.cols
    }
}
