use crate::tensor::Tensor;

pub fn gd(x: &Tensor<f64>, y: &Tensor<f64>, w: &Tensor<f64>, l: f64) -> Tensor<f64> {
    let data_size = *(y.get_shape().get(0).unwrap()) as f64;

    let prediction = x.mul(w).unwrap();
    let loss = y.sub(&prediction).unwrap();
    
    let d = x
        .t()
        .mul(&loss)
        .unwrap()
        .scale(-1.0 / data_size);

    let learning_adjusted_d = d.scale(l);

    w.sub(&learning_adjusted_d).unwrap()
}
