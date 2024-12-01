mod algebra;
mod calculus;
mod geometry;

use std::f64::consts::PI;

fn main() {
    let poly_result = algebra::solve_polynomial(vec![1.0, -2.0, 1.0]);
    println!("Polynomial roots: {:?}", poly_result);

    let derivative = calculus::derivative(2.0);
    println!("Derivative at x=2: {}", derivative);

    let volume = geometry::calculate_complex_volume(5.0, 3.0, PI/4.0);
    println!("Complex volume: {}", volume);
}