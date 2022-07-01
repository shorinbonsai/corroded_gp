use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::Lines;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)] // copy is to allow non-consumption when initializing multiple gp's...
pub struct Data {
    // NOTE! Outputs to be predicted is assumed to be the last column!
    dimensions: usize,
    train: Vec<Vec<f32>>,
    test: Vec<Vec<f32>>,
    // to [j][i], e.g. test[0] gets the first variable for all instances ;)
}

///Getter and Utility methods for the data
impl Data {
    pub fn train(&self) -> &Vec<Vec<f32>> {
        &self.train
    }
    pub fn test(&self) -> &Vec<Vec<f32>> {
        &self.test
    }
    pub fn dimensions(&self) -> usize {
        self.dimensions
    }
    pub fn train_targets(&self) -> &Vec<f32> {
        &self.train[self.train.len() - 1]
    }
    pub fn test_targets(&self) -> &Vec<f32> {
        &self.test[self.test.len() - 1]
    }

    ///"constructor"
    pub fn new(dataset: &str, test_split: f32) -> Data {
        let (train_data, test_data) = splitdata(dataset, test_split);
        let dimension = train_data.len() - 1;
        Data {
            dimensions: dimension,
            train: train_data,
            test: test_data,
        }
    }
}

///Private method to transpose the 2D Vector
fn transpose_array(array: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut result: Vec<Vec<f32>> = vec![];
    for j in 0..array[0].len() {
        let mut column = vec![];
        for i in 0..array.len() {
            column.push(array[i][j]);
        }
        result.push(column);
    }
    result
}

///Primary helper method to parse and split the dataset
/// Returns a tuple of the training and testing data
fn splitdata(filename: &str, split: f32) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let reader = BufReader::new(File::open(filename).expect("Cannot open file"));
    let mut rows: Vec<Vec<f32>> = vec![];
    for line in reader.lines() {
        let mut tmpy: Vec<f32> = Vec::new();
        for word in line.unwrap().split_whitespace() {
            let wordy = word.parse::<f32>().unwrap();
            tmpy.push(wordy);
        }
        if !&tmpy.is_empty() {
            rows.push(tmpy.clone());
        }
    }
    let n = rows.len();
    let n_test = ((n as f32) * split) as usize;
    let n_train = n - n_test;
    let mut indices: Vec<usize> = (0..n).collect();
    indices.shuffle(&mut thread_rng());

    let mut training: Vec<Vec<f32>> = vec![];
    let mut testing: Vec<Vec<f32>> = vec![];
    for i in (0..n) {
        if i < n_train {
            let tmp = indices[i];
            let tmp2 = rows[tmp].clone();
            training.push(tmp2);
        } else {
            let tmp = indices[i];
            let tmp2 = rows[tmp].clone();
            testing.push(tmp2);
        }
    }
    let transposetrain = transpose_array(training);
    let transposetest = transpose_array(testing);
    (transposetrain, transposetest)
}
