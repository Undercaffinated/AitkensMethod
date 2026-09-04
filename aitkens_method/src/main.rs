// Defined by the
const P0: f64 = 1.0;

fn main() {
    println!("Hello, world!");

    // a and the following for loop construct the sequence required by 5.1
    let mut a: Vec<f64> = Vec::with_capacity(11);
    a.push(P0);
    for i in 0..10 {
        a.push(alpha(a[i]));
    }

    // b and the following for loop construct the sequence required by 5.2
    let mut b: Vec<f64> = Vec::with_capacity(10);

}

/// This is effectively a constant, but the compiler doesn't like running sqrt() at compile time.
/// It's such a small anti-optimization that it isn't worth worrying about, but that's what this is.
fn p() -> f64 {
    (1.0 + 13.0_f64.sqrt()) / 2.0
}

/// Calculates p_(n+1) term for sequence α as defined in overleaf document.
fn alpha(pn: f64) -> f64 {
    (3.0 + pn).sqrt()
}

fn beta(pn: f64) -> f64 {
    ((alpha(pn) - p()) / (pn - p())).abs()
}

fn gamma(pn: f64) -> f64 {
    pn - (alpha(pn) - pn).powi(2) / (alpha(alpha(pn)) - 2.0 * alpha(pn) + pn)
}

/// This function is intended only as a helper method for delta(). There is a really messy term "p
/// hat_(n+1)" that I want to extract and construct separately. So here it is!
fn _gamma_next(pn: f64) -> f64 {
    gamma(alpha(pn))
}

fn delta(pn: f64) -> f64 {
    ((_gamma_next(pn)_gamma_next - p()) / (pn - p())).abs()
}
