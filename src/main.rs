use std::env;

struct HW {
    n: f64,
    freqs: [f64; 3],
    p: f64,
    q: f64,
    heterozygous: f64,
}

impl HW {
    #[allow(non_snake_case)]
    pub fn new(AA: f64, Aa: f64, aa: f64) -> Self {
        let n = AA + Aa + aa;
        let (AA, Aa, aa) = (AA / n, Aa / n, aa / n);
        Self {
            n,
            freqs: [AA, Aa, aa],
            p: AA + (0.5 * Aa),
            q: aa + (0.5 * Aa),
            heterozygous: Aa,
        }
    }
}

fn main() {
    let args: Vec<f64> = env::args().flat_map(|s| s.parse()).collect();
    if args.len() != 3 {
        println!("Usage: hwcalc AA_count Aa_count aa_count");
    } else {
        let hw = HW::new(args[0], args[1], args[2]);

        println!("N = {:.0}", hw.n);
        println!("p(A) = {:.2} (frequency of A)", hw.p);
        println!("q(a) = {:.2} (frequency of a)", hw.q);
        println!("Observed genotype frequencies: {:.2?}", hw.freqs);

        let hw_2pq = 2.0 * hw.p * hw.q;

        println!("p^2 = {:.2} (next generation AA under random mating)", hw.p.powi(2));
        println!("2pq = {:.2} (next generation Aa)", hw_2pq);
        println!("q^2 = {:.2} (next generation aa)", hw.q.powi(2));

        let f_st = (hw_2pq - hw.heterozygous) / hw_2pq;
        println!("F_ST = {:.2}", f_st);
    }
}
