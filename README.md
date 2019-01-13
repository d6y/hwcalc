# Compute [Hardy-Weinberg Equilibrium](https://en.wikipedia.org/wiki/Hardy%E2%80%93Weinberg_principle) Values

Call with allele counts for AA Aa aa on the command line:

```
$ cargo run 664 435 116
   Compiling hwcalc v0.1.0 (/Users/richard/Developer/rust/hwcalc)
    Finished dev [unoptimized + debuginfo] target(s) in 0.98s
     Running `target/debug/hwcalc 664 435 116`
N = 1215
p(A) = 0.73 (frequency of A)
q(a) = 0.27 (frequency of a)
Observed genotype frequencies: [0.55, 0.36, 0.10]
p^2 = 0.53 (next generation AA under random mating)
2pq = 0.40 (next generation Aa)
q^2 = 0.08 (next generation aa)
F_ST = 0.10
```


