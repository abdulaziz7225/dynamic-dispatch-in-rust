# Getting Comfortable with `Box<dyn Trait>` in Rust

## Objective

Create a program that demonstrates the use of `Box<dyn Trait>` for dynamic dispatch in Rust.

### Instructions

1. **Define a Trait**

   - Create a trait named `Shape` with methods for calculating the area and perimeter of a shape.

2. **Implement the Trait**

   - Define at least two structs (e.g., `Circle` and `Rectangle`) that implement the `Shape` trait.
   - Ensure each struct has the necessary fields to calculate area and perimeter.

3. **Use `Box<dyn Shape>`**

   - Create a vector that holds `Box<dyn Shape>` to store different shapes.
   - Add instances of your `Circle` and `Rectangle` structs to this vector.

4. **Iterate and Display**

   - Write a loop that iterates over the vector of shapes and calls the methods to calculate and print the area and perimeter for each shape.

5. **Experiment**
   - Add another shape (e.g., `Triangle`) that also implements the `Shape` trait.
   - Modify the program to handle the new shape and display its area and perimeter.
