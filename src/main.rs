use std::env;

#[allow(non_snake_case)]
fn main() {

    let args: Vec<f64> = env::args().flat_map(|s| s.parse()).collect();
    if args.len() != 3 {
        println!("Usage: hwcalc AA_count Aa_count aa_count");
    } else {
        let total: f64 = args.iter().sum();
        let freq: Vec<f64> = args.iter().map(|c| c / total).collect();
        println!("Observed genotype frequencies: {:.2?}", freq);

        let (AA, Aa, aa) = (freq[0], freq[1], freq[2]);
        let p = AA + (0.5 * Aa);
        let q = aa + (0.5 * Aa);

        println!("p(A) = {:.2}", p);
        println!("q(a) = {:.2}", q);

        let hw_2pq = 2.0 * p * q;

        println!("p^2 = {:.2}", p.powi(2) );
        println!("2pq = {:.2}", hw_2pq);
        println!("q^2 = {:.2}", q.powi(2));

        let F_ST = (hw_2pq - Aa) / hw_2pq;
        println!("F_ST = {:.2}", F_ST);
    }
}
