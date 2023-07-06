# MAHF BMF

A collection of common continuous benchmark functions for [MAHF](https://github.com/mahf-opt/mahf).

All functions are scaled to a domain of [-1, 1] for each dimension.

# Getting Started

Add the following to your `Cargo.toml`:

```toml
[dependencies]
mahf = { git = "https://github.com/mahf-opt/mahf" }
mahf_bmf = { git = "https://github.com/mahf-opt/mahf-bmf" }
```

Constructing a problem instance and evaluating a solution:

```rust
use mahf::problems::ObjectiveFunction;
use mahf_bmf::BenchmarkFunction;

let problem = BenchmarkFunction::sphere(/*dim: */ 30);
let x = vec![0.1; 30];
println!("f({:?}) = {:?}", x, problem.objective(&x));
```

# References

The benchmark functions were taken from the following sources:

- [1] [BenchmarkFcns Toolbox](https://mazhar-ansari-ardeh.github.io/BenchmarkFcns/)
- [2] [Virtual Library of Simulation Experiments](https://www.sfu.ca/~ssurjano/optimization.html)
- [3] Momin Jamil and Xin-She Yang. 2013. A Literature Survey of Benchmark Functions For Global Optimization Problems.
  IJMMNO 4, 2 (2013), 150. DOI:https://doi.org/10/gh48qq

# License

This project is licensed under
the [GNU General Public License v3.0](https://github.com/mahf-opt/mahf/blob/master/LICENSE).
