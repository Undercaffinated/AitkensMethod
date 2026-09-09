
fn main() {
    // Limit of {p_n} as n -> ∞
    let l: f64 = (1.0 + 13.0_f64.sqrt()) / 2.0;
    println!("lim = {}", l);
    println!();


    // Req. 5.1 - Generate the initial series {p_n} where p_0 = 1 and p_{n+1} = (3.0 + p_n).sqrt()
    let mut p_n: Vec<f64> = Vec::with_capacity(11);

    p_n.push(1.0);
    for i in 0..10 {
        p_n.push(pn_next(p_n[i]));
    }
    custom_print("p_n", &p_n);

    // The next most reasonable steps are to generate the rest of the
    // basic series(es) we will need for the remainder of the project.
    // Here, namely "Delta p_n" and "Delta Squared p_n" which will be
    // named in the code d_pn and d2_pn. These will also be referred to
    // as the first and second difference sequences of p_n respectively.
    let d_pn: Vec<f64> = difference_series(&p_n);
    let d2_pn: Vec<f64> = difference_series(&d_pn);

    // Ok, look, names suck. And math makes some of the least comprehensible names
    // of all time. This whole next block is to define "Delta p_n Squared", not to be
    // confused with "Delta Squared p_n". This series is defined as the following:
    // {Delta Squared p_n} = {Delta p_n}^2. We shall use the name d_pn2 to represent
    // Delta p_n Squared.
    let mut d_pn2: Vec<f64> = Vec::with_capacity(d_pn.len());
    for i in 0..d_pn.len() {
        d_pn2.push(d_pn[i].powi(2));
    }

    // Req. 5.2 - Generate the sequence {ϵ}
    // Epsilon of n := Absolute value of the ratio of the difference between the
    // n+1 elemnt and the limit of that series (numerator) and the difference between
    // the nth term of the series and the limit (denominator).
    let epsilon_pn: Vec<f64> = erf(&p_n);
    custom_print("ε_n", &epsilon_pn);

    // Req. 5.3 - Generate the sequence {p_hat_n}
    let p_hat = aitken_method(&p_n, &d_pn, &d2_pn);
    custom_print("p_hat", &p_hat);

    // Req. 5.4 - Generate the sequence {e_hat_n}
    let e_hat_n: Vec<f64> = erf(&p_hat);
    custom_print("ε_hat_n", &e_hat_n);

    // Req. 5.5 - Generate the sequence {column 5}
    // This is a patch job. There wasn't any allusion to this columbn in
    // the project, until I noticed there was an extra column in the demo table.
    let mut col_5: Vec<f64> = Vec::with_capacity(p_hat.len());
    for i in 0..p_hat.len() {
        col_5.push(((p_hat[i] - l)/(p_n[i] - l)).abs());
    }
    custom_print("col_5", &col_5);

}


/// Calculates the successive term of {p_n}.
fn pn_next(pn: f64) -> f64 {
    (3.0 + pn).sqrt()
}

/// Given a sequence with n >= 2, returns a sequence where ds[n] = p[n+1] - p[n].
/// As a note, this means the difference sequence will contain one less element
/// than its predecessor.
fn difference_series(series: &Vec<f64>) -> Vec<f64> {
    let mut ds: Vec<f64> = Vec::with_capacity(series.len() - 1);
    for i in 0..series.len() - 1 {
        ds.push(series[i+1] - series[i]);
    }
    ds
}

/// The nth term of {p_hat} is equal to the difference between
/// the nth element of the original sequence {p_n} and the ratio between the squares of the
/// first difference sequence of p_n, (Delta p_n) and the second difference sequence from p_n.
/// In other words, the nth term of p_hat = p_n - (delta_p_n)^2 / (delta squared P_n).
fn aitken_method(base: &Vec<f64>, first_difference: &Vec<f64>, second_difference: &Vec<f64>) -> Vec<f64> {
    let mut am: Vec<f64> = Vec::with_capacity(second_difference.len());
    for i in 0..second_difference.len() {
        am.push(base[i] - (first_difference[i]).powi(2) / (second_difference[i]));
    }
    am
}

/// Generates a list whose elements are the absolute value of the ratio of the error generated 
/// between subsequent terms of the given sequence and the limit of that sequence, l.
fn erf(seq: &Vec<f64>) -> Vec<f64> {
    let l: f64 = (1.0 + 13.0_f64.sqrt()) / 2.0;
    let mut v: Vec<f64> = Vec::with_capacity(seq.len() -1);
        for i in 0..seq.len() - 1 {
            v.push(((seq[i+1] - l) / (seq[i] - l)).abs());
        }
    v
}

/// Custom printer for this project
/// Prints a given sequence to the console.
fn custom_print(name: &str, seq: &Vec<f64>) {
    for i in 0..seq.len() {
        println!("{}{} = {}", name, i, seq[i]);
    }
    println!();
}
