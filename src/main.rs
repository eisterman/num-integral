extern crate argparse;
extern crate meval;

#[macro_use]
extern crate generator;

use argparse::{ArgumentParser, Store};
use meval::Expr;
use generator::Gn;

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
        let result = ap.parse_args();
        if let Err(c) = result {
            let args: Vec<String> = std::env::args().collect();
            let name = if args.len() > 0 { &args[0][..] } else { "unknown" };
            ap.print_help(name, &mut std::io::stdout()).unwrap();
            std::process::exit(c);
        }
    }
    let expr: Expr = text_func.parse().unwrap();
    let func = expr.bind("x").unwrap();

    let g = Gn::new_scoped(|mut s| {
        let mut i = low_bound.clone();
        while i < high_bound {
            i += epsilon;
            s.yield_(i);
        }
        done!();
    });

    let result: f64 = g.map(&*func).fold(0_f64, |acc, x| acc + (x * epsilon) );
    println!("Result: {}", result);
}
