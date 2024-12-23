pub fn get_upper_coeffs<'a>(f: fn(f64) -> f64, mesh: &'a[f64]) -> Vec<f64>/*-> Box<dyn Fn(f64) -> f64> */ {

    let n = mesh.len();
    let mut triangle: Vec<Vec<f64>> = Vec::with_capacity(n);
    for i in (1..=n).rev() {
        triangle.push(Vec::with_capacity(i));
    }

    for i in 0..n {
        triangle[i].push(f(mesh[i]));
    }

    for k in 1..=n {
        for j in 0..n-k {
            let value = (triangle[j+1][k-1] - triangle[j][k-1])/(mesh[j+k] - mesh[j]);
            triangle[j].push(value);
        }
    }
    triangle[0].clone()
}

fn get_upper_coeffs_from_table<'a>(func_values: &'a[f64], mesh: &'a[f64]) -> Vec<f64> {
    let n = mesh.len();
    let mut triangle: Vec<Vec<f64>> = Vec::with_capacity(n);
    for i in (1..=n).rev() {
        triangle.push(Vec::with_capacity(i));
    }

    for i in 0..n {triangle[i].push(func_values[i]);}

    for k in 1..=n {
        for j in 0..n-k {
            let value = (triangle[j+1][k-1] - triangle[j][k-1])/(mesh[j+k] - mesh[j]);
            triangle[j].push(value);
        }
    }
    triangle[0].clone()
}

pub fn set_up_poly<'a>(upper_coeffs: &Vec<f64>, mesh: &'a[f64]) -> Box<dyn Fn(f64) -> f64 + 'a> {
    let mesh_clone = mesh/* .clone()*/;
    let upper_coeffs_clone = upper_coeffs.clone();
    Box::new(move |x: f64| -> f64 {
        let mut result = upper_coeffs_clone[0];
        let mut term = 1.0;
        for k in 1..mesh_clone.len() {
            term *= x - mesh_clone[k-1];
            result += upper_coeffs_clone[k]*term;
        }
        result
    })
}

pub fn interpolate_by_newton<'a>(f: fn(f64) -> f64, mesh: &'a[f64]) -> Box<dyn Fn(f64) -> f64 + 'a> {
    let upper_coeffs = get_upper_coeffs(f, mesh);
    set_up_poly(&upper_coeffs, mesh)
}

pub fn interpolate_by_newton_from_table<'a>(func_values: &'a[f64], mesh: &'a[f64]) -> Box<dyn Fn(f64) -> f64 + 'a> {
    // let upper_coeffs = get_upper_coeffs(|x| func_values[mesh.iter().position(|&v| v==x)])

    let upper_coeffs = get_upper_coeffs_from_table(func_values, mesh);
    set_up_poly(&upper_coeffs, mesh)

} 