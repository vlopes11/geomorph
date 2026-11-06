# Geomorph

[![CI](https://github.com/vlopes11/geomorph/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/vlopes11/geomorph/actions/workflows/ci.yml)
[![Latest version](https://img.shields.io/crates/v/geomorph.svg)](https://crates.io/crates/geomorph)
[![Documentation](https://docs.rs/geomorph/badge.svg)](https://docs.rs/geomorph)

Simple conversion between different coordinate systems without external wrappers injection

# Example

```rust
use geomorph::{Coord, Mgrs, MgrsPrecision, Utm};

let lat: f64 = -23.0095839;
let lon: f64 = -43.4361816;

let coord = Coord::new(lat, lon);
let utm = Utm::from(coord);
let mgrs = Mgrs::from_latlon(lat, lon);

println!("coord: {}", coord);
println!("utm: {}", utm);
println!(
    "mgrs precision 10km: {}",
    mgrs.with_precision(MgrsPrecision::P10km)
)
```
