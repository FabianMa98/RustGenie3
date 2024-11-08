use std::fs::File;
use bio::io::fasta;
// Think about what input I should take, if i either take a many files input
// or just a Count Matrix, which should make more sense
// Think about developing a suite, where mapping processes are started from within

struct Reference {
    file_path: &str,
    data: String,
}