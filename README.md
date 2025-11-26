# Linear Regression in Rust with Python Prediction

This project demonstrates how to train a simple linear regression model in Rust using a CSV dataset, save the model parameters to a JSON file, and use Python to load the model and make predictions.

## Project Structure
```
Linear_Regression/
│
├── housing.csv
├── linreg_rust/
│ ├── Cargo.toml
│ └── src/
│ └── main.rs
└── predict.py
```

## Requirements

### Rust
- Rust toolchain (rustup)
- Visual Studio Build Tools (for MSVC linker)

### Python
- Python 3.x

No external Python packages are required.

## Dataset

The synthetic dataset file `housing.csv` contains two columns:

```
area,price
1810.8904159657686,601.8110241841596
3827.5000724347065,1245.154292710809
3061.9787963399176,992.3542348566266
```


Place `housing.csv` in the project root folder (`Linear_Regression`).

## Rust Code

The Rust program reads the CSV file, computes the linear regression parameters, and saves them to `model.json`.

Run the program:
```
cd linreg_rust
cargo run
```


After running, a file named `model.json` will be created inside `linreg_rust`:
```
{
"slope": <value>,
"intercept": <value>
}
```


## Python Prediction Script

The Python script loads `model.json` and predicts the price based on a given area.

Run:
```
python predict.py
```


Example usage:

```
Enter area to predict price for: 1810.8904159657686
Predicted price: 592.7444825966379
Original price: 601.8110241841596
```


## How It Works

1. Rust reads `housing.csv` and extracts area and price values.
2. Rust computes:
   - slope
   - intercept
3. Rust saves the model to `model.json`.
4. Python loads the JSON model.
5. Python computes predicted price using:
```
price = slope * area + intercept
```

## Notes

- This project uses simple linear regression.
- The model is easily extensible to multiple variables if needed.
- The Python script always reads the latest generated model.

## License

This project is provided for learning and demonstration purposes.
