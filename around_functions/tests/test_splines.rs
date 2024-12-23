use around_functions::interpolation::splines::*;
use around_functions::*;
use slae::gauss::gauss_elimination;
use slae::tridiagonal::solve_tridiagonal;

fn f(x: f64) -> f64 {
    x * x
}
fn f_prime(x: f64) -> f64 {
    2.0 * x
}

#[test]
fn test_splines() {
    let a = 0.; let b = 1.; let n: usize = 7;
    let f_prime_a = f_prime(a); let f_prime_b = f_prime(b);
    let m = 3;
    let mesh = equidistant_nodes(a, b, n);

    let mut base_fn: fn(f64) -> f64  = beta_3;

    // let s = find_spline(f, a, b, f_prime(a), f_prime(b), &mesh, m);

    let mut alphas: Vec<f64> = find_coeffs(f, a, b, f_prime_a, f_prime_b, &mesh, m);
    println!("\nalphas.len = {}", alphas.len());
    let mesh_clone = mesh.clone();
    let s = Box::new(move |x: f64| {
        let mut s = 0.;
        let n = mesh_clone.len()-1;
        let h = (b-a)/(n as f64);
        for k in 0..=n+2 {s += alphas[k] * base_fn((x - a - h*(k as f64  -1.))/h);}
        s

    });

    println!("f(x) : ");
    for i in 0..mesh.len() {print!("{:.4}, ", f(mesh[i]));}
    println!("\ns(x) : ");
    for i in 0..mesh.len() {print!("{:.4}, ", s(mesh[i]));}

    
    // let mut matrix = vec![vec![0.; n+3]; n+3];
    // let mut matrix: Vec<Vec<f64>> = Vec::with_capacity(n+3);
    // for i in 0..n+3 {
    //     matrix.push(Vec::with_capacity(n+3));
    //     for j in 0..n+3 {matrix[i].push(0.);}
    // } 

    // for j  in 0..=n+2 {
    //     matrix[0][j] = beta_prime_3(-(j as f64) + 1.);
    //     matrix[n+2][j] = beta_prime_3(n as f64 - j as f64 +1.);
    // }

    // for i in 1..n+2 {
    //     for j in 0..=n+2 {matrix[i][j] = base_fn(i as f64 - j as f64);}
    // }

    // // let mut d = vec![0.; n+3];
    // let mut d: Vec<f64> = Vec::with_capacity(n+3);
    // for i in 0..n+3 {d.push(0.);}

    // d[0] = f_prime_a; d[n+2] = f_prime_b;
    // for i in 0..=n {d[i+1] = f(mesh[i]);}

    // for i in 0..n+3 {
    //     for j in 0..n+3 {
    //         print!("{:.4} ", matrix[i][j]);
    //     }
    //     println!();
    // }

    // let alphas = gauss_elimination(&matrix, &d);
    // for k in 0..alphas.len() {print!("{}, ", alphas[k]);}

    


    // let mut c: Vec<f64> = Vec::with_capacity(n+3);
    // let mut a: Vec<f64> = Vec::with_capacity(n+2);
    // let mut b: Vec<f64> = Vec::with_capacity(n+2);
    // let mut d: Vec<f64> = Vec::with_capacity(n+3);
    
    // let k = -base_fn(1.)/beta_prime_3(1.);
    // c.push(beta_prime_3(-1.) * k + base_fn(-1.));
    // b.push(beta_prime_3(0.) * k + base_fn(0.)); 
    // d.push(f_prime_a * k + f(mesh[0]));
    
    // for i in 0..=n {
    //     a.push(base_fn(-1.));
    //     c.push(base_fn(0.));
    //     b.push(base_fn(1.));
    //     d.push(f(mesh[i]));
    // }
    // let k = -base_fn(-1.)/beta_prime_3(-1.);
    // a.push(beta_prime_3(0.) * k + base_fn(0.));
    // c.push(beta_prime_3(1.) * k + base_fn(1.));
    // d.push(f_prime_b * k + f(mesh[n]));

    //  print!("a = [");
    // for i in 0..n+2 {print!("{:.4}, ", a[i]);}
    // print!("\nc = [");
    // for i in 0..n+3 {print!("{:.4}, ", c[i]);}
    // print!("\nb = [");
    // for i in 0..n+2 {print!("{:.4}, ", b[i]);}
    // print!("\nd = [");
    // for i in 0..n+3 {print!("{:.4}, ", d[i]);}

}
