fn get_coefficients_and_weights(n: usize) -> (Vec<f64>, Vec<f64>) {
    match n {
        1 => {
            let abscissas = vec![-0.577350269189626, 0.577350269189626];
            let weights = vec![1., 1.];
            (abscissas, weights)
        },
        2 => {
            let abscissas = vec![-0.774596669241483, 0., 0.774596669241483];
            let weights = vec![0.555555555555556, 0.888888888888889, 0.555555555555556];
            (abscissas, weights)
        },
        3 => {
            let abscissas = vec![-0.861136311594053, -0.339981043584856, 0.339981043584856, 0.861136311594053];
            let weights = vec![0.347854845137454, 0.652145154862546, 0.652145154862546, 0.347854845137454];
            (abscissas, weights)
        },
        _ => {panic!("Unsupported order")}
    }
}

pub fn integrate_by_gauss(f: fn(f64) -> f64, a: f64, b: f64, degree: usize, epsilon: f64, max_count: usize) -> (f64, usize) {
    let n = (degree-1)/2;
    let (abscissas, weights) = get_coefficients_and_weights(n);
    let mut number = n+1;
    let mut integ = 0.; let mut integ_twice = 0.;
    loop {
        let h = (b-a)/(number-1) as f64;
        for i in 0..number-1 {
            let x = a + (i as f64)*h;
            let points: Vec<f64> = abscissas.iter().map(|ksi| h*(ksi+1.)/2. + x).collect();
            // for node in points.iter() {print!("{:.4} ", node);} println!();
            let value_ith = (h/2.) * points.iter().enumerate().map(|(i, &node)| weights[i]*f(node)).sum::<f64>();
            // println!("value_ith = {:.4}", value_ith);
            integ_twice += value_ith;
        }
        // println!("number = {}, integ_twice = {:.4}", number, integ_twice);
        if (integ_twice - integ).abs() < epsilon {return (integ_twice, number);}
        if number>=max_count {println!("max_count reached! might not be convergent!"); break;}
        integ = integ_twice;
        integ_twice = 0.;
        number *= 2;
    }

    (integ_twice, number)
}