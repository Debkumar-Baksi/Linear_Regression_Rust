import json

# Load Rust model from JSON file
with open("linreg_rust/model.json", "r") as f:
    model = json.load(f)

slope = model["slope"]
intercept = model["intercept"]

def predict(area):
    """Predict price using linear regression model."""
    return slope * area + intercept

# ---- TESTING ----

print("Loaded Model:")
print(f"Slope: {slope}")
print(f"Intercept: {intercept}")
print()

# Ask user for area
try:
    area = float(input("Enter area to predict price for: "))
    price = predict(area)
    print(f"Predicted price for area {area}: {price}")
except ValueError:
    print("Invalid number entered.")
