mod matrix;
mod rf;

use matrix::GeneExpressionMatrix;
use rf::gene_regulators_intersection;

use randomforest::criterion::Mse;
use randomforest::RandomForestRegressorOptions;
use randomforest::table::TableBuilder;
fn main() {


    let features = [
    &[0.0, 2.0, 1.0, 0.0][..],
    &[0.0, 2.0, 1.0, 1.0][..],
    &[1.0, 2.0, 1.0, 0.0][..],
    &[2.0, 1.0, 1.0, 0.0][..],
    &[2.0, 0.0, 0.0, 0.0][..],
    &[2.0, 0.0, 0.0, 1.0][..],
    &[1.0, 0.0, 0.0, 1.0][..],
    &[0.0, 1.0, 1.0, 0.0][..],
    &[0.0, 0.0, 0.0, 0.0][..],
    &[2.0, 1.0, 0.0, 0.0][..],
    &[0.0, 1.0, 0.0, 1.0][..],
    &[1.0, 1.0, 1.0, 1.0][..],
    ];
    let target = [
    25.0, 30.0, 46.0, 45.0, 52.0, 23.0, 43.0, 35.0, 38.0, 46.0, 48.0, 52.0
    ];

    let mut table_builder = TableBuilder::new();
    for (xs, y) in features.iter().zip(target.iter()) {
        table_builder.add_row(xs, *y).unwrap();
    }
    let table = table_builder.build().unwrap();

    let regressor = RandomForestRegressorOptions::new()
        .seed(0)
        .fit(Mse, table);

    assert_eq!(regressor.predict(&[1.0, 2.0, 0.0, 0.0]), 41.9785);
    println!("{}", regressor.predict(&[1.0, 0.0, 1.0, 1.0]));
    // Define some sample genes and samples
    let genes = vec!["Gene1", "Gene2", "Gene3"];
    let samples = vec!["SampleA", "SampleB", "SampleC"];

    // Initialize the gene expression matrix
    let mut matrix = GeneExpressionMatrix::new(genes, samples);

    // Set some values
    matrix.set("Gene1", "SampleA", 1.2).unwrap();
    matrix.set("Gene2", "SampleB", 3.4).unwrap();

    // Retrieve a value
    if let Some(value) = matrix.get("Gene1", "SampleA") {
        println!("Expression level of Gene1 in SampleA: {}", value);
    }

    // Normalize by gene
    matrix.normalize_by_gene();

    // Retrieve expression levels for a specific gene
    if let Some(gene_expr) = matrix.get_gene_expression("Gene1") {
        println!("Expression levels for Gene1 across samples: {:?}", gene_expr);
    }
    let genes: Vec<&str> = vec!["Gene1", "Gene2", "Gene3"];
    let regulators: Vec<&str> = vec!["Gene1"];
    let test: Vec<usize> = gene_regulators_intersection(regulators, genes);
    println!("{:?}", test)
}
