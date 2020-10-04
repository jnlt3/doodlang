use crate::eq_solver::function::{Function};
use crate::eq_solver::matrix::Matrix;
use crate::doodlang::parser::FunctionManager;
use crate::print_vec;
use rand::{thread_rng, Rng};

pub struct PolyApprox {
    degree: usize,
}

impl PolyApprox {
    pub fn new(degree: usize) -> Self {
        PolyApprox {
            degree,
        }
    }

    pub fn approx1d(&mut self, function_manager: &mut Box<FunctionManager>, function: &mut Box<dyn Function>, parameter: &str, range: (f64, f64), num_samples: usize) -> Matrix {
        let diff = range.1 - range.0;
        let mut in_v = Vec::with_capacity(num_samples);
        let mut out_v = Vec::with_capacity(num_samples);
        let id = function_manager.ids.get(parameter).unwrap();
        for i in 0..num_samples {
            let val = range.0 + (i as f64) * diff / ((num_samples - 1) as f64);
            function_manager.variables[*id] = val;
            in_v.push(val);
            out_v.push(function.output(function_manager));
        }
        let mut vandermonde_matrix = Matrix::zeros(self.degree + 1, num_samples);
        for i in 0..vandermonde_matrix.columns {
            for j in 0..vandermonde_matrix.rows {
                vandermonde_matrix[(i, j)] = in_v[j].powf(i as f64);
            }
        }
        let out = Matrix {
            values: out_v,
            columns: 1,
            rows: num_samples,
        };
        let transpose = &vandermonde_matrix.transpose();
        return ((transpose * vandermonde_matrix).invert() * transpose).transpose() * out;
    }
}

pub struct SecantSolver {
    range: [f64; 2],
    derivative_step: f64,
    attempt_num: usize,
    iter: usize,
    margin: f64,
}

impl SecantSolver {
    pub fn new(init_guess_range: [f64; 2], derivative_step: f64, attempt_num: usize, iter: usize, margin: f64) -> SecantSolver {
        SecantSolver {
            range: init_guess_range,
            derivative_step,
            attempt_num,
            iter,
            margin,
        }
    }

    pub fn solve_for_param(&mut self, func: &mut Box<dyn Function>, func_manager: &mut Box<FunctionManager>, param: &str) -> f64 {
        let id = func_manager.ids.get(param).unwrap();
        let start_val = func_manager.variables[*id];
        let mut lowest_error = f64::INFINITY;
        let mut zero_val = Option::None;
        for _i in 0..self.attempt_num {
            let mut point = self.range[0] + thread_rng().gen::<f64>() * (self.range[1] - self.range[0]);
            for _j in 0..self.iter {
                func_manager.variables[*id] = point;
                let current_val = func.output(&func_manager);
                let err = current_val.abs();
                if err < lowest_error {
                    lowest_error = err;
                    zero_val = Option::Some(point);
                    if err <= self.margin {
                        return zero_val.unwrap();
                    }
                }
                func_manager.variables[*id] = point - self.derivative_step;
                let prev = func.output(func_manager);
                func_manager.variables[*id] = point + self.derivative_step;
                let derivative = (func.output(func_manager) - prev) / (self.derivative_step * 2f64);
                point -= current_val / derivative;
            }
        }
        func_manager.variables[*id] = start_val;
        return zero_val.unwrap();
    }
}

pub struct QuadFind {
    range: [f64; 2],
    regress_range: f64,
    attempt_num: usize,
    iter: usize,
    margin: f64,
    poly: PolyApprox,
}

impl QuadFind {
    pub fn new(init_guess_range: [f64; 2], regress_range: f64, attempt_num: usize, iter: usize, margin: f64) -> QuadFind {
        QuadFind {
            range: init_guess_range,
            regress_range,
            attempt_num,
            iter,
            margin,
            poly: PolyApprox {
                degree: 2,
            },
        }
    }

    pub fn solve_for_param(&mut self, func: &mut Box<dyn Function>, func_manager: &mut Box<FunctionManager>, param: &str) -> f64 {
        let id = *func_manager.ids.get(param).unwrap();
        let mut lowest_error = f64::INFINITY;
        let mut zero_val = Option::None;
        for _i in 0..self.attempt_num {
            let mut point = self.range[0] + thread_rng().gen::<f64>() * (self.range[1] - self.range[0]);
            for _j in 0..self.iter {
                let current_val = func.output(func_manager);
                let err = current_val.abs();
                if err < lowest_error {
                    lowest_error = err;
                    zero_val = Option::Some(point);
                    if err <= self.margin {
                        return zero_val.unwrap();
                    }
                }
                let coeffs = self.poly.approx1d(
                    func_manager,
                    func,
                    param,
                    (point + self.regress_range, point - self.regress_range),
                    10,
                );
                let a = coeffs[(0, 0)];
                let b = coeffs[(1, 0)];
                let c = coeffs[(2, 0)];
                let delta = -(b.powi(2) - 4f64 * a * c);
                if delta < 0f64 {
                    if a < 0f64 {
                        let point0 = point - self.regress_range;
                        let point1 = point + self.regress_range;
                        func_manager.variables[id] = point0;
                        let val0 = func.output(func_manager);
                        func_manager.variables[id] = point1;
                        let val1 = func.output(func_manager);
                        point = if val0 < val1 { point0 } else { point1 };
                    } else {
                        point = -b / (2f64 * a);
                        func_manager.variables[id] = point;
                    }
                } else {
                    let sqrt_delta = delta.sqrt();
                    let root0 = (-coeffs[(1, 0)] + sqrt_delta) / (2f64 * a);
                    let root1 = (-coeffs[(1, 0)] - sqrt_delta) / (2f64 * a);
                    func_manager.variables[id] = root0;
                    let val0 = func.output(func_manager);
                    func_manager.variables[id] = root1;
                    let val1 = func.output(func_manager);
                    point = if val0 < val1 { root0 } else { root1 };
                }
            }
        }
        return zero_val.unwrap();
    }
}