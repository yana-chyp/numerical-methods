use around_functions::{equidistant_nodes, interpolation::trigonometry};

#[test]
fn test_coeffs() {
    let n = 5;
    let mesh = equidistant_nodes(0., 2.*std::f64::consts::PI, n);
    let a_coeffs = trigonometry::get_a_k(f, &mesh);
    let b_coeffs = trigonometry::get_b_k(f, &mesh);

    for i in 0..a_coeffs.len() {println!("a{} = {}", i, a_coeffs[i]);}
    for i in 0..b_coeffs.len() {println!("b{} = {}", i, b_coeffs[i]);}

}

fn f(x: f64) -> f64 {
    3.* (15.*x).cos()
}