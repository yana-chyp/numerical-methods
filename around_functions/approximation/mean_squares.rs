pub fn get_best_approx_from_function(f: fn(f64) -> f64, mesh: &Vec<f64>, m: usize) -> Box<dyn Fn(f64) -> f64> {
    let matrix = get_matrix(&mesh, m);
    let r = get_vector_from_function(f, mesh, m);
    let a = slae::gauss::gauss_elimination(&matrix, &r);
    
    //might be commented if not needed
    println!("a coeffs: ");
    for i in 0..a.len() {print!("{:.4} ", a[i]);} println!();
    //

    Box::new(move |x: f64| 
        a.iter().enumerate().
        map(|(i, &a_i)| a_i * x.powf(i as f64)).sum())

}

pub fn get_best_approx_from_table(f_values: &Vec<f64>, mesh: &Vec<f64>, m: usize) -> Box<dyn Fn(f64) -> f64> {
    let matrix = get_matrix(&mesh, m);
    let r = get_vector_from_table(&f_values, &mesh, m);
    let a = slae::gauss::gauss_elimination(&matrix, &r);
    
    //might be commented if not needed
    println!("a coeffs: ");
    for i in 0..a.len() {print!("{:.4} ", a[i]);} println!();
    //

    Box::new(move |x: f64| 
        a.iter().enumerate().
        map(|(i, &a_i)| a_i * x.powf(i as f64)).sum())
}

fn get_matrix(mesh: &Vec<f64>, m: usize) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.; m+1]; m+1];

    for i in 0..=m {
        for j in 0..=m {
            matrix[i][j] = mesh.iter().map(|x| x.powf((i+j) as f64)).sum();
        }
    }

    matrix
}

fn get_vector_from_function(f: fn(f64) -> f64, mesh: &Vec<f64>, m: usize) -> Vec<f64> {
    // let r = (0..=m).iter().map(|i| mesh.iter().map(|x:| f(x) * x.powf(i as f64)).sum()).collect()
    let mut r = vec![0.; m+1];
    for i in 0..=m {
        r[i] = mesh.iter().map(|x| f(*x) * x.powf(i as f64)).sum();
    }
    r
}

fn get_vector_from_table(f_values: &Vec<f64>, mesh: &Vec<f64>, m: usize) -> Vec<f64> {
    let mut r = vec![0.; m+1];
    for i in 0..=m {
        r[i] = mesh.iter().enumerate().map(|(k, &x)| f_values[k] * x.powf(i as f64)).sum();
    }
    r
}
