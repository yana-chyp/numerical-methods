use around_functions::*;

fn f(x: f64) -> f64 {
    x.abs()
}

#[test]
fn test_mean_squares() {
    let a = -3.; let b = 3.;
    let n = 4; let m = 2;
    // let mesh = interpolation::equidistant_nodes(a, b, n);
    let mesh = vec![-3., -1., 0., 1., 3.];
    let f_values = vec![-4., -0.8, 1.6, 2.3, 1.5];

    // let approximation = best_approximation::get_best_approx_from_table(&f_values, &mesh, m);
    // let approximation = least_squares::get_best_approx_from_function(f, &mesh, m);


    // let matrix = best_approximation::get_matrix(&mesh, m);
    // let r = best_approximation::get_vector_from_function(f, &mesh, m);
    // let r = best_approximation::get_vector_from_table(&f_values, &mesh, m);


    // for i in 0..mesh.len() {print!("{:.4} ", mesh[i]);} println!();
    // for i in 0..matrix.len() {
    //     for j in 0..matrix[i].len() {
    //         print!("{:.4} ", matrix[i][j]);
    //     }
    //     println!();
    // }
    // for i in 0..r.len() {print!("{:.4} ", r[i]);} println!();



}