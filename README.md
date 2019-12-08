## GeoMorph

[![Build Status](https://travis-ci.org/vlopes11/geomorph.svg?branch=master)](https://travis-ci.org/vlopes11/geomorph)
[![Latest version](https://img.shields.io/crates/v/geomorph.svg)](https://crates.io/crates/geomorph)
[![Documentation](https://docs.rs/geomorph/badge.svg)](https://docs.rs/geomorph)

Simple conversion between different coordinate systems without external wrappers injection

# Code Example
```
use geomorph::*;

fn main() {
    let lat: f64 = -23.0095839;
    let lon: f64 = -43.4361816;
    
    let coord = coord::Coord::new(lat, lon);
    let utm: Utm = coord.clone().into();
    println!("coord: {}", coord);
    println!("utm: {}", utm);
    // Will print:
    //  coord: (-23.0095839, -43.4361816)
    //  utm: 23K 660265 7454564
}
```
