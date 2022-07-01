use std::cmp;

///Takes as input a vector of f32's x containing outputs of tree on inputs
/// as well as vector of correct expected outputs
/// returns a float for root mean squared error
pub fn root_mean_squared(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    assert!(x.len() == y.len());

    let numb = x.len() as f32;
    let value = x
        .into_iter()
        .zip(y.into_iter())
        .map(|(x, y)| ((x - y) as f32).powi(2));
    let error_sum: f32 = value.sum();
    (error_sum / numb).sqrt()
}

///Takes as input a vector of f32's x containing outputs of tree on inputs
/// as well as vector of correct expected outputs
/// returns a float for sum of absolute errors
pub fn sae(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    assert!(x.len() == y.len());
    let n = x.len() as f32;
    let value = x
        .into_iter()
        .zip(y.into_iter())
        .map(|(x, y)| ((x - y) as f32).abs());
    let sae: f32 = value.sum();
    sae
}

///Element by element addition between two vectors
/// returns vector of outputs
pub fn add(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    assert!(x.len() == y.len());
    x.into_iter()
        .zip(y.into_iter())
        .map(|(x, y)| x + y)
        .collect()
}

///Element by element subtraction between two vectors
/// returns vector of outputs
pub fn subtract(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    assert!(x.len() == y.len());
    x.into_iter()
        .zip(y.into_iter())
        .map(|(x, y)| x - y)
        .collect()
}

///Element by element product between two vectors
/// returns vector of outputs
pub fn multiply(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    assert!(x.len() == y.len());
    x.into_iter() // or rayon::prelude::par_iter()
        .zip(y.into_iter())
        .map(|(x, y)| x * y)
        .collect()
}

///Element by element quotient between two vectors
/// returns vector of outputs
pub fn divide(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    assert!(x.len() == y.len());

    let min = 0.00001;
    let protected_division = |(num, denom): (f32, f32)| -> f32 {
        // numerator and denominator
        if denom.abs() > min {
            return num / denom;
        } else {
            return num;
        }
    };
    x.into_iter()
        .zip(y.into_iter())
        .map(protected_division)
        .collect()
}
///Element by element sine function calculation
/// returns a vector of outputs
pub fn sine(x: Vec<f32>) -> Vec<f32> {
    x.into_iter().map(|x| x.sin()).collect()
}

///Element by element natural logarithm calculation
/// returns a vector of outputs
pub fn ln(x: Vec<f32>) -> Vec<f32> {
    x.into_iter().map(|x| x.ln()).collect()
}

///Element by element natural squaring calculation
/// returns a vector of outputs
pub fn square(x: Vec<f32>) -> Vec<f32> {
    let e = |x: f32| -> f32 { x.powf(2.0) };
    x.into_iter().map(e).collect()
}
