pub fn middle_rectangles(f: fn(f64) -> f64, a: f64, b: f64, max_count: usize, epsilon: f64) -> (f64, usize) {
    let mut n = 1;
    let mut integ_n = 0.; let mut integ_2n = 0.;

    let h = (b-a)/(n as f64);
    for i in 1..=n {
        let x = a + h * ((i as f64) - 0.5);
        integ_n += f(x);
    }
    integ_n *= h;
    
    loop {
        n *= 2;
        let h = (b-a)/(n as f64);
        for i in 1..=n {
            let x = a + h * ((i as f64) - 0.5);
            integ_2n += f(x);
        }
        integ_2n *= h;
        if (integ_2n - integ_n).abs() < epsilon {return (integ_2n, n)}
        if n >= max_count {println!("max_count reached! might not be convergent!"); break;}
        integ_n = integ_2n;
        integ_2n = 0.;
    }

    (integ_2n, n)
}

pub fn trapezia(f: fn(f64) -> f64, a: f64, b: f64, max_count: usize, epsilon: f64) -> (f64, usize) {
    let mut n = 1;
    let mut integ_n = 0.; let mut integ_2n = 0.;

    let mut sum_f = (f(a) + f(b))/2.;
    let h = (b-a)/(n as f64);
    for i in 1..n {
        let x = a + h*(i as f64);
        sum_f += f(x);
    }
    integ_n = h * sum_f;
    // println!("integ_n: {}, n = {}", integ_n, n);

    loop {
        n *= 2;
        let h = (b-a)/(n as f64);
        integ_2n = (f(a) + f(b))/2.;
        for i in 1..n {
            let x = a + h * (i as f64);
            integ_2n += f(x);
        }
        
        integ_2n *= h;
        // println!("integ_2n: {}, n = {}, h = {}, sum_f = {}", integ_2n, n, h, sum_f);


        if (integ_2n - integ_n).abs() < epsilon {return (integ_2n, n)}
        if n >= max_count {println!("max_count reached! might not be convergent!"); break;}
        integ_n = integ_2n;
        integ_2n = 0.;

    }
    // println!("before return : integ_2n: {}, n = {}", integ_2n, n);

    (integ_2n, n)
}

pub fn simpson_rule(f: fn(f64) -> f64, a: f64, b: f64, max_count: usize, epsilon: f64) -> (f64, usize) {
    let mut n = 1;
    let mut integ_n = 0.; let mut integ_2n = 0.;

    loop {
        // println!("integ_n: {}, n = {}", integ_n, n);
        n = 2*n+1;
        let h = (b-a)/((n-1) as f64);
        for i in 0..(n-1)/2 {
            let x = a + 2.*h*(i as f64);
            integ_2n += f(x) + 4.*f(x+h) + f(x+2.*h);
        }
        integ_2n *= h/3.;
        if (integ_2n - integ_n).abs() < epsilon {return (integ_2n, n);}
        if n >= max_count {println!("max_count reached! might not be convergent!"); break;}
        integ_n = integ_2n;
        integ_2n = 0.;

    }

    (integ_2n, n)
}