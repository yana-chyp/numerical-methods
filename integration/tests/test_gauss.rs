use integration::{gauss::*, newton_cotes::*};

fn f1(x: f64) -> f64 {
    x.exp()
}

fn f2(x: f64) -> f64 {
    1./(1.+x*x)
}

fn f3(x: f64) -> f64 {
    1./(1.+x)
}

fn F1(x: f64) -> f64 {
    x.exp()
}

fn F2(x: f64) -> f64 {
    x.atan()
}

fn F3(x: f64) -> f64 {
    (x+1.).ln()
}

#[test]
fn test_guass() {
    let epsilon = 1e-4; let max_count = 100;
    let degree = 4;
    
    
    let a = -1.; let b = 1.;
    let integ_exact = F1(b) - F1(a);
    
    let (value, number) = integrate_by_gauss(f1, a, b, degree, epsilon, max_count);
    println!("gauss:");
    println!("for y = exp(x) \t approx = {:.5}, n = {} \t error = {}", value, number, (integ_exact - value).abs());
    println!("middle_rectangles:");
    let (integ_approx, n) = middle_rectangles(f1, a, b, max_count, epsilon);
    println!("for y = exp(x) \t approx = {:.4}, n = {} \t error = {}", integ_approx, n, (integ_exact - integ_approx).abs());
    println!("trapezia:");
    let (integ_approx, n) = trapezia(f1, a, b, max_count, epsilon);
    println!("for y = exp(x) \t approx = {:.4}, n = {} \t error = {}", integ_approx, n, (integ_exact - integ_approx).abs());
    println!("simpson:");
    let (integ_approx, n) = simpson_rule(f1, a, b, max_count, epsilon);
    println!("for y = exp(x) \t approx = {:.4}, n = {} \t error = {}", integ_approx, n, (integ_exact - integ_approx).abs());


    let a = 0.; let b = 1.;
    let integ_exact = F2(b) - F2(a);

    println!("\n\ngauss:");
    let (value, number) = integrate_by_gauss(f2, a, b, degree, epsilon, max_count);
    println!("for y = 1/(1+x^2) \t approx = {:.5}, n = {} \t error = {}", value, number, (integ_exact - value).abs());
    println!("middle_rectangles:");
    let (integ_approx, n) = middle_rectangles(f2, a, b, max_count, epsilon);
    println!("for y = 1/(1+x^2) \t approx = {:.4}, n = {} \t error = {}", integ_approx, n, (integ_exact - integ_approx).abs());
    println!("trapezia:");
    let (integ_approx, n) = trapezia(f2, a, b, max_count, epsilon);
    println!("for y = 1/(1+x^2) \t approx = {:.4}, n = {} \t error = {}", integ_approx, n, (integ_exact - integ_approx).abs());
    println!("simpson:");
    let (integ_approx, n) = simpson_rule(f2, a, b, max_count, epsilon);
    println!("for y = 1/(1+x^2) \t approx = {:.4}, n = {} \t error = {}", integ_approx, n, (integ_exact - integ_approx).abs());


    let a = 1.; let b = 3.;
    let integ_exact = F3(b) - F3(a);
    println!("\n\ngauss:");
    let (value, number) = integrate_by_gauss(f3, a, b, degree, epsilon, max_count);
    println!("for y = 1/(1+x) \t approx = {:.5}, n = {} \t error = {}", value, number, ((F3(b) - F3(a)) - value).abs());
    println!("middle_rectangles:");
    let (integ_approx, n) = middle_rectangles(f3, a, b, max_count, epsilon);
    println!("for y = 1/(1+x) \t approx = {:.4}, n = {} \t error = {}", integ_approx, n, (integ_exact - integ_approx).abs());
    println!("trapezia:");
    let (integ_approx, n) = trapezia(f3, a, b, max_count, epsilon);
    println!("for y = 1/(1+x) \t approx = {:.4}, n = {} \t error = {}", integ_approx, n, (integ_exact - integ_approx).abs());
    println!("simpson:");
    let (integ_approx, n) = simpson_rule(f3, a, b, max_count, epsilon);
    println!("for y = 1/(1+x) \t approx = {:.4}, n = {} \t error = {}", integ_approx, n, (integ_exact - integ_approx).abs());


}