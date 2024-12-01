pub fn solve_polynomial(coefficients: Vec<f64>) -> Vec<f64> {
    let mut roots = Vec::new();
    
    if coefficients.len() == 3 {
        let a = coefficients[0];
        let b = coefficients[1];
        let c = coefficients[2];
        
        let discriminant = b*b - 4.0*a*c;
        if discriminant >= 0.0 {
            roots.push((-b + discriminant.sqrt()) / (2.0*a));
            roots.push((-b - discriminant.sqrt()) / (2.0*a));
        }
    }
    
    roots
}