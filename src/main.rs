extern crate argparse;
extern crate meval;
extern crate crossbeam;
extern crate num_cpus;

use argparse::{ArgumentParser, Store};
use meval::Expr;
use std::iter::Iterator;

struct FloatRange {
    value: f64,
    highbound: f64,
    epsilon: f64,
}

impl FloatRange {
    fn new(lowbound: f64, highbound: f64, epsilon: f64) -> Result<FloatRange,()> {
        if lowbound <= highbound {
            Ok(FloatRange{ value: lowbound - epsilon, highbound, epsilon})
        } else {
            Err(())
        }
    }
}

impl Iterator for FloatRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.value += self.epsilon;
        if self.value < self.highbound {
            Some(self.value)
        } else {
            None
        }
    }
}

fn main() {
    let mut text_func = "x^2".to_string();
    let mut low_bound = 0_f64;
    let mut high_bound = 1_f64;
    let mut epsilon = 1e-4_f64;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Calculate numerical monodimensional intwgrals.");
        ap.refer(&mut text_func)
            .add_argument("function", Store,
                          "Integrand function. Use \"\" for enclose the function and x as incognita.")
            .required();
        ap.refer(&mut low_bound)
            .add_argument("low_bound", Store,
                          "Lower integral bound.")
            .required();
        ap.refer(&mut high_bound)
            .add_argument("high_bound", Store,
                          "Higher integral bound.")
            .required();
        ap.refer(&mut epsilon)
            .add_option(&["-e", "--epsilon"], Store,
                        "Numerical epsilon/step.");
        ap.parse_args_or_exit();
    }
    let expr: Expr = text_func.parse().unwrap();

    let threads = num_cpus::get();

    let mut results = vec![0_f64;threads];
    {
        let refs: Vec<&mut f64> = results.iter_mut().collect();
        crossbeam::scope(|spawner| {
            for (i,out_ref) in refs.into_iter().enumerate(){
                let lb = low_bound + (high_bound - low_bound)/(threads as f64) * (i as f64);
                let hb = low_bound + (high_bound - low_bound)/(threads as f64) * (i as f64 + 1_f64);
                let eps = epsilon.clone();
                let g = FloatRange::new(lb, hb, eps).unwrap();
                let ex = expr.clone();
                spawner.spawn(move ||{
                    let func = ex.bind("x").unwrap();
                    let partial = g.map(&*func).fold(0_f64, |acc, x| acc + (x * epsilon) );
                    *out_ref += partial;
                });
            }
        });
    }

    let output: f64 = results.iter().sum();
    println!("Result: {}", output);
}
