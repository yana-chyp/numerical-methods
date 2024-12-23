use slae::tridiagonal::*;
use slae::gauss::*;


pub fn beta_1(x: f64) -> f64 {
    let mut result = 0.;
    if x.abs()<=1. {result = 1. - x.abs();}
    result
}

pub fn beta_3(x: f64) -> f64 {
    let mut result = 0.;
    if x.abs()<=1. {result = ( (2. - x.abs()).powf(3.) - 4.*(1. - x.abs()).powf(3.)) /6.;}
    else if x.abs()<=2. && x.abs()>1. {result = (2. - x.abs()).powf(3.) / 6.;}
    result
}

pub fn beta_prime_3(x: f64) -> f64 {
    let mut result = 0.;
    if x.abs()<=1. {result = ( 3. * (-x.signum())*(2. - x.abs()).powf(2.) - 12.*(-x.signum())*(1. - x.abs()).powf(2.)) /6.;}
    else if x.abs()<=2. && x.abs()>1. {result = 3.* (-x.signum())*(2. - x.abs()).powf(2.) / 6.;}
    result
} 

fn beta_m(x: f64) -> f64 {
    //return beta_0 for now
    if x.abs()<=0.5 {return 1.;}
    return 0.;
}


pub fn find_spline<'a>(f: fn(f64) -> f64, a: f64, b: f64, f_prime_a: f64, f_prime_b: f64, mesh: &'a Vec<f64>, m: usize) -> Box<dyn Fn(f64) -> f64 + 'a> {
    Box::new(move |x: f64| {
        let mut s = 0.;
        let n = mesh.len()-1;
        let mut alphas: Vec<f64> = find_coeffs(f, a, b, f_prime_a, f_prime_b, mesh, m);
        let base_fn = match m {
            1 => beta_1,
            3 => beta_3,
            _ => beta_m,
        };

        let h = (b-a)/(n as f64);

        match m {
            1 =>  {
                for k in 0..=n {s += alphas[k] * base_fn((x - mesh[k])/h);}
            },
            3 => {
                for k in 0..=n+2 {s += alphas[k] * base_fn((x - a - h*(k as f64 - 1.))/h);}
            },
            _ => {}
        }
        s
    
        
    }) 
}

pub fn find_coeffs(f: fn(f64) -> f64, a: f64, b: f64, f_prime_a: f64, f_prime_b: f64, mesh: &Vec<f64>, m: usize) -> Vec<f64> {
    let n = mesh.len()-1;
    let mut alphas: Vec<f64> = Vec::with_capacity(n+3);
    let mut base_fn: fn(f64) -> f64  = beta_m;
    let h = (b-a)/(n as f64);

        
        match m {
            1 => {
                base_fn = beta_1;
                for i in 0..=n {alphas.push(f(mesh[i]));}
            },
            3 => {
                base_fn = beta_3;
                let mut c_vec: Vec<f64> = Vec::with_capacity(n+3);
                let mut a_vec: Vec<f64> = Vec::with_capacity(n+2);
                let mut b_vec: Vec<f64> = Vec::with_capacity(n+2);
                let mut d_vec: Vec<f64> = Vec::with_capacity(n+3);
                
                let k = -base_fn(1.)/beta_prime_3(1.);
                c_vec.push(beta_prime_3(-1.) * k + base_fn(-1.));
                b_vec.push(beta_prime_3(0.) * k + base_fn(0.)); 
                d_vec.push(f_prime_a * h * k + f(a));
                
                for i in 0..=n {
                    a_vec.push(base_fn(-1.));
                    c_vec.push(base_fn(0.));
                    b_vec.push(base_fn(1.));
                    d_vec.push(f(mesh[i]));
                }
                let k = -base_fn(-1.)/beta_prime_3(-1.);
                a_vec.push(beta_prime_3(0.) * k + base_fn(0.));
                c_vec.push(beta_prime_3(1.) * k + base_fn(1.));
                d_vec.push(f_prime_b * h * k + f(b));

                
    //  print!("a = [");
    // for i in 0..n+2 {print!("{:.4}, ", a_vec[i]);}
    // print!("]\nc = [");
    // for i in 0..n+3 {print!("{:.4}, ", c_vec[i]);}
    // print!("]\nb = [");
    // for i in 0..n+2 {print!("{:.4}, ", b_vec[i]);}
    // print!("]\nd = [");
    // for i in 0..n+3 {print!("{:.4}, ", d_vec[i]);}

                alphas = solve_tridiagonal(&a_vec, &c_vec, &b_vec, &d_vec);

            },
            _ => panic!("Wrong m")
        }
        alphas
}
