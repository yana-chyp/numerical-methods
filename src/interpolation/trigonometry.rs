pub fn get_a_k(f: fn(f64) -> f64, mesh: &[f64]) -> Vec<f64> {
    let n = mesh.len()-1;
    match n%2 {
        1 => {
            let n = (n-1)/2;
            let mut a_coeffs: Vec<f64> = Vec::with_capacity(n+1);
            for k in 0..=n {
                let mut a_k = 0.0;
                for j in 0..=2*n {
                    //check if the result is not rounded to usize
                    a_k += f(mesh[j])*(((2*j*k) as f64 / (2*n + 1) as f64) * std::f64::consts::PI).cos();
                }
                a_k *= 2./(2.*n as f64+1.);
                a_coeffs.push(a_k);
            }
            a_coeffs
        },
        _ => {
            let n = n/2;
            let mut a_coeffs: Vec<f64> = Vec::with_capacity(n+1);
            for k in 0..=n {
                let mut a_k = 0.0;
                for j in 0..=2*n-1 {
                    a_k += f(mesh[j]) * (((j*k) as f64 /n as f64) * std::f64::consts::PI).cos();
                }
                a_k /= n as f64;
                a_coeffs.push(a_k);
            }
            a_coeffs
        }
    }
}

pub fn get_b_k(f: fn(f64) -> f64, mesh: &[f64]) -> Vec<f64> {
    let n = mesh.len()-1;
    match n%2 {
        1 => {
            let n = (n-1)/2;
            let mut b_coeffs: Vec<f64> = Vec::with_capacity(n);
            for k in 0..=n {
                let mut b_k = 0.0;
                for j in 0..=2*n {
                    b_k += f(mesh[j])*(((2*j*k) as f64 / (2*n + 1) as f64) * std::f64::consts::PI).sin();
                }
                b_k *= (2./(2.*n as f64+1.)) as f64;
                b_coeffs.push(b_k);
            }
            b_coeffs
        },
        _ => {
            let n = n/2;
            let mut b_coeffs: Vec<f64> = Vec::with_capacity(n-1);
            for k in 1..=n-1 {
                let mut b_k = 0.0;
                for j in 0..=2*n-1 {
                    b_k += f(mesh[j]) * (((j*k) as f64 /n as f64) * std::f64::consts::PI).sin();
                }
                b_k /= n as f64;
                b_coeffs.push(b_k);
            }

            b_coeffs
        }
    }
}

pub fn set_up_poly<'a>(coeffs_a_k: &Vec<f64>, coeffs_b_k: &Vec<f64>, mesh: &'a[f64]) -> Box<dyn Fn(f64) -> f64 + 'a> {
    let a_k = coeffs_a_k.clone();
    let b_k = coeffs_b_k.clone(); 
    Box::new(move |t: f64| {
        let mut q = 0.;
        let n = mesh.len()-1;
        match n%2 {
            1 => {
                let n = (n-1)/2;
                q = a_k[0]/2.;
                for k in 1..=n {
                    q += a_k[k] * (k as f64 * t).cos() + b_k[k] * (k as f64 * t).sin();
                }
            }
            _ => {
                let n = n/2;
                q = a_k[0]/2.;
                for k in 1..=n-1 {
                    q += a_k[k] * (k as f64 * t).cos() + b_k[k-1] * (k as f64 * t).sin();
                }
                q += a_k[n]/2. * (n as f64 * t).cos();
            }
        }
        q
    })
}

pub fn interpolate_periodic<'a>(f : fn(f64) -> f64, mesh: &'a[f64]) -> Box<dyn Fn(f64) -> f64 + 'a> {
    let mesh_clone = mesh.clone();
    let coeffs_a = get_a_k(f, &mesh_clone);
    for i in 0..coeffs_a.len() {println!("a[{}] = {}", i, coeffs_a[i]);}
    let coeffs_b = get_b_k(f, &mesh_clone);
    for i in 0..coeffs_b.len() {println!("b[{}] = {}", i, coeffs_b[i]);}
    set_up_poly(&coeffs_a, &coeffs_b, &mesh_clone)
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