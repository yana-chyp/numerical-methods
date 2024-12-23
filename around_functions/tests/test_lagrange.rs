
#[test]
fn test_lagrange() {
    let a = -1.0; let b = 1.0; let n = 10;
    let mesh = around_functions::equidistant_nodes(a, b, n);

    // println!("mesh");
    // for x in &mesh {print!("{:.4} ", x);}

    // println!("\nf(x)\n");
    // for x in &mesh {print!("{:.4} ", f(*x));}
    
    // println!("\n\n");

    // move |x: f64| {
    //     let mut interp_func =  0.0;
    //     for i in 0..mesh.len() {
    //         let mut l_i = Box::new(|_: f64| 1.0);

    //         for j in 0..mesh.len() {
    //             if i!=j {
    //                 let old_l_i = l_i;
    //                 l_i = Box::new(move |x| {
    //                     (x-mesh[j])/(mesh[i] - mesh[j]) * old_l_i(x)
    //                 });
    //             }
    //             interp_func += f(mesh[i]) * l_i(x);
    //         }
    //     }
    //     interp_func
    // }

    let interp_func = around_functions::interpolation::lagrange::interpolate_by_lagrange(f, &mesh);

    for x in &mesh {println!("{}\t{}", interp_func(*x), f(*x));}

}


fn f(x: f64) -> f64 {
    (x+2.0).ln()
}
