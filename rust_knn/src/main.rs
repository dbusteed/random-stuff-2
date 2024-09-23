use std::error::Error;
use std::fs::File;
use csv::{ReaderBuilder};
use ndarray::prelude::*;

fn main() {
    let data = read_csv("data/insurance_reg.csv").unwrap();

    let split = ((data.shape()[0] as f64) * 0.80).floor() as usize;
    let train = data.slice(s![..split, ..]);
    let test = data.slice(s![split.., ..]);

    let ks = vec![1, 3, 5];

    for k in ks {
        let mut y_hats: Vec<f64> = Vec::new();
        for i in 0..test.shape()[0] {
            let features = test.slice(s![i, 1..]).to_owned();
            let mut distances: Vec<(&f64, f64)> = Vec::new();
            for j in 0..train.shape()[0] {
                let dist = distance(&features, train.slice(s![j, 1..]).to_owned());
                distances.push((&train[[j, 0]], dist));
            }
            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let y_hat: f64 = distances.iter().take(k).map(|x| x.0).sum::<f64>() / k as f64;
            y_hats.push(y_hat);
        }
        
        let y_hats = Array::from_vec(y_hats);
        let err = mae(test.slice(s![.., 0]).to_owned(), y_hats);
    
        println!("K={}: {}", k, err);
    }
}

fn read_csv(filename: &str) -> Result<Array2<f64>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    
    let mut data: Vec<f64> = Vec::new();
    let mut ncol = 0;
    let mut nrow = 0;
    for res in rdr.records() {
        let record = res?;
        let row: Vec<f64> = record.iter().map(|r| r.parse().unwrap()).collect();
        if nrow == 0 {
            nrow = row.len();
        }
        data.extend(row);
        ncol += 1;
    }

    let data: Array2<f64> = Array::from_vec(data).into_shape((ncol, nrow)).unwrap();
    Ok(data)
}

fn distance(arr1: &Array1<f64>, arr2: Array1<f64>) -> f64 {
    let diff = arr1 - arr2;
    let diff_sq = &diff * &diff;  // TODO better way to square array?
    diff_sq.sum().sqrt()
}

fn mae(y_true: Array1<f64>, y_pred: Array1<f64>) -> f64 {
    let err = y_true - y_pred;
    (err.sum() / err.len() as f64).abs()
}