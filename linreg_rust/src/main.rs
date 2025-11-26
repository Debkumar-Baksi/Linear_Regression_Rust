use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct Model {
    slope: f64,
    intercept: f64,
}

fn read_csv(path: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let x: f64 = record[0].parse()?;
        let y: f64 = record[1].parse()?;
        xs.push(x);
        ys.push(y);
    }
    Ok((xs, ys))
}

fn linear_regression(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let n = xs.len() as f64;

    let sum_x: f64 = xs.iter().sum();
    let sum_y: f64 = ys.iter().sum();
    let sum_xy: f64 = xs.iter().zip(ys).map(|(x, y)| x * y).sum();
    let sum_x2: f64 = xs.iter().map(|x| x * x).sum();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / n;

    (slope, intercept)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Reading dataset...");
    let (xs, ys) = read_csv("../housing.csv")?;

    println!("Training model...");
    let (slope, intercept) = linear_regression(&xs, &ys);

    let model = Model { slope, intercept };
    std::fs::write("model.json", serde_json::to_string_pretty(&model)?)?;

    println!("Model saved to model.json");
    println!("Slope: {}", slope);
    println!("Intercept: {}", intercept);

    Ok(())
}
