# Compute [Hardy-Weinberg Equilibrium](https://en.wikipedia.org/wiki/Hardy%E2%80%93Weinberg_principle) Values

Call with allele counts for AA Aa aa  on the command line:

```
$ cargo run 664 435 116
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/hwcalc 664 435 116`
Observed genotype frequencies: [0.55, 0.36, 0.10]
p^2 = 0.53
2pq = 0.40
q^2 = 0.08
F_ST = 0.10
```


