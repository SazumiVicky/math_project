pub fn calculate_complex_volume(radius: f64, height: f64, angle: f64) -> f64 {
    let cylinder_vol = std::f64::consts::PI * radius.powi(2) * height;
    let cone_vol = (1.0/3.0) * std::f64::consts::PI * radius.powi(2) * height * angle.sin();
    
    cylinder_vol + cone_vol
}