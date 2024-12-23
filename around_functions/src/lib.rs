pub mod interpolation;
pub mod approximation;


//idea: calculate cofficients separately in modules, 
//then write the function here to multiply them by basis functions 
//and so evaluate function

pub fn equidistant_nodes(a: f64, b: f64, n: usize) -> Vec<f64> {
    let h = (b-a)/(n as f64);
    // let mesh: Vec<64> = (0..=n).map(|i| a + i*h).collect();
    let mut mesh: Vec<f64> = Vec::with_capacity(n+1);
    for i in 0..=n {
        mesh.push(a+(i as f64)*h);
    }
    mesh
}

pub fn chebyshev_nodes(a: f64, b: f64, n: usize) -> Vec<f64> {
    let mut mesh: Vec<f64> = Vec::with_capacity(n);
    for i in 1..=n {
        mesh.push((a+b)/2. + (b-a)/2. * f64::cos(((2*i-1) as f64) * std::f64::consts::PI/((2*n) as f64)));
    }
    mesh
}

pub fn find_max_error<'a>(rnf: Box<dyn Fn(f64) -> f64 + 'a>, mesh: &'a[f64], number_submesh: usize) -> (f64, f64) {
    let mut max_error = 0.;
    let mut x_max_error = 0.;
    for i in 0..mesh.len()-1 {
        let h = (mesh[i+1]-mesh[i])/number_submesh as f64;
        for j in 1..number_submesh-1 {
            let x = mesh[i]+(j as f64)*h;
            let value = rnf(x).abs();
            if value > max_error {x_max_error = x; max_error = value;}
        }
    }   
    (x_max_error, max_error) 
}
