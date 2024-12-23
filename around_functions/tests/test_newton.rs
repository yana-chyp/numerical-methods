use around_functions::interpolation::newton::*;

// #[cfg(test)]
#[test]
fn test_newton() {
    let a = 1.; let b = 2.; let n = 4;
    let mesh = around_functions::equidistant_nodes(a, b, n);

    let upper_coeffs = get_upper_coeffs(g, &mesh);

    let newton_poly = set_up_poly(&upper_coeffs, &mesh);

    for x in &mesh {println!("{}\t{}", newton_poly(*x), g(*x));}
    print!("\n");


    // for i in 0..upper_coeffs.len() {
    //     print!("{:.4} ", upper_coeffs[i]);
    // }

    // println!("the len of mesh {}", mesh.len());
    // let D = interpolate_by_newton(g, &mesh);

    // let n = mesh.len();
    // // println!("n = {}", n);

    // let mut triangle: Vec<Vec<f64>> = Vec::with_capacity(n);
    // // println!("( before ) triangle.len = {}", triangle.len());
    

    // for i in (1..=n).rev() {
    //     // println!("i = {}", i);
    //     // println!("( inside ) triangle.len = {}", triangle.len());
    //     triangle.push(Vec::with_capacity(i));
    // }

    // // println!("( after ) triangle.len = {}", triangle.len());

    // for i in 0..n {
    //     triangle[i].push(g(mesh[i]));
    //     // println!("len of triangle[i] = {}", triangle[i].len());
    // }
    // for k in 1..=n {
    //     for j in 0..n-k {
    //         let value = (triangle[j+1][k-1] - triangle[j][k-1])/(mesh[j+k] - mesh[j]);
    //         // print!("triangle[{}][{}] = {:.4} ", j, k, value);
    //         triangle[j].push(value);
    //     }
    //     // print!("\n");
    //     // println!("len of triangle[i] = {}", triangle[].len());
    // }
    
    // for i in 0..triangle.len() {
    //     // println!("len of triangle[{}] = {}", i, triangle[i].len());
    // }

    // for row in &triangle {
    //     for element in row {
    //         print!("{:.4} ", element);
    //     }
    //     print!("\n");
        
    // }
}



fn g(x: f64) -> f64 {
    (x+2.0).ln()
}