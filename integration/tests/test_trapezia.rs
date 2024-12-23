use integration::newton_cotes::*;

fn f1(x: f64) -> f64 {
    x.exp()
}

fn f2(x: f64) -> f64 {
    1./(1. + x*x)
}

fn f3(x: f64) -> f64 {
    1./(1. + x)
}

fn F1(x: f64) -> f64 {
    x.exp()
}

fn F2(x: f64) -> f64 {
    x.atan()
}

fn F3(x: f64) -> f64 {
    (1. + x).ln()
}

#[test]
fn test_trapezia() {
    let max_count = 1000; let epsilon = 1e-5;

    let a = -1.; let b = 1.;
    let integ_exact = F1(b) - F1(a);
    let (integ_approx, n) = trapezia(f1, a, b, max_count, epsilon);
    println!("for f(x) = exp(x) approx = {:.4}, n = {} \twhile exact = {}", integ_approx, n, integ_exact);
    println!("error = {:.8}\n", (integ_exact - integ_approx).abs());

    let a = 0.; let b = 1.;
    let integ_exact = F2(b) - F2(a);
    let (integ_approx, n) = trapezia(f2, a, b, max_count, epsilon);
    println!("for f(x) = 1/(1+x^2) approx = {:.4}, n = {} \twhile exact = {}", integ_approx, n, integ_exact);
    println!("error = {:.8}\n", (integ_exact - integ_approx).abs());



    let a = 1.; let b = 3.;
    let integ_exact = F3(b) - F3(a);
    let (integ_approx, n) = trapezia(f3, a, b, max_count, epsilon);
    println!("for f(x) = 1/(1+x) approx = {:.4}, n = {} \twhile exact = {}", integ_approx, n, integ_exact);
    println!("error = {:.8}\n", (integ_exact - integ_approx).abs());



}