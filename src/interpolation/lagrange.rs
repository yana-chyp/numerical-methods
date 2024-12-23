pub fn interpolate_by_lagrange<'a>(f: fn(f64) -> f64, mesh: &'a[f64]) -> Box<dyn Fn(f64) -> f64 + 'a> {
    // let mut interp_func = |x| 0.0;
    let mesh_clone = mesh/* .clone()*/;

    Box::new(move |x: f64| {
        let mut interp_func =  0.0;

        for i in 0..mesh_clone.len() {
            let mut l_i = 1.0;

            for j in 0..mesh_clone.len() {
                if i!=j {
                    l_i *= (x - mesh_clone[j])/(mesh_clone[i] - mesh_clone[j]);
                }
            }
            interp_func += f(mesh_clone[i]) * l_i;
        }
        interp_func
    })

    // for i in 0..mesh.len() {
    //     let mut l_i = |x: f64| 1.0;
    //     for j in 0..i {
    //         let l_i = |x| (x-mesh[j])/(mesh[i] - mesh[j]) * l_i(x);
    //     }
    //     for j in i+1..mesh.len() {
    //         let l_i = |x| (x-mesh[j])/(mesh[i] - mesh[j]) * l_i(x);
    //     }
    //     let interp_func = |x| interp_func(x) + l_i(x) * f(mesh[i]);
    // }
    // interp_func
}

