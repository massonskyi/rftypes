use pyo3::prelude::*;
#[pyclass]
pub struct Filter {
    num: Vec<f32>,
    de_num: Vec<f32>,
    state: Vec<f32>,
}

#[pymethods]
impl Filter {
    #[new]
    fn new(num: Vec<f32>, de_num: Vec<f32>) -> Self {
        let state = vec![0.0; num.len() - 1];
        Self { num, de_num, state }
    }

    fn step(&mut self, mut input_data: Vec<f32>, output: &mut Vec<f32>) {
        let mut p_num;
        let mut p_de_num;
        let mut p_state;
        let mut p_sig = input_data.iter_mut();
        let mut p_out = output.iter_mut();

        // Check that the size of the numerator and denominator coefficient arrays match the expected size.
        if self.num.len() > 1 {
            for i in 0..input_data.len() {
                p_num = self.num.iter_mut();
                p_de_num = self.de_num.iter_mut();
                p_state = self.state.iter_mut();

                if let (Some(state), Some(num), Some(sig)) = (p_state.next(), p_num.next(), p_sig.next()) {
                    *p_out.next().unwrap() = state + num * &sig;
                }
                p_num.nth(0);
                p_de_num.nth(0);

                // Fill in middle delays
                for _ in 0..self.num.len() - 2 {
                    *p_state.nth(0).unwrap() = *p_state.nth(1).unwrap()
                        + *p_sig.clone().nth(i).unwrap() * *p_num.nth(0).unwrap()
                        *p_out.clone().cloned().nth(i).unwrap() * *p_de_num.next().unwrap();


                    if let Some(state) = p_state.next() {
                        *state;
                    }
                    p_num.next();
                    p_de_num.next();
                }

                // Calculate last delay
                *p_state.nth(0).unwrap() = *p_sig.nth(i).unwrap() * *p_num.nth(0).unwrap()
                    - *p_out.clone().nth(i).unwrap() * *p_de_num.nth(0).unwrap();
            }
        } else {
            let mut p_num = self.num.iter();

            for i in 0..input_data.len() {
                *p_out.nth(i).unwrap() = *p_sig.nth(i).unwrap() * *p_num.nth(0).unwrap();
            }
        }
    }
}