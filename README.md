## GeoMorph

[![Build Status](https://travis-ci.org/vlopes11/geomorph.svg?branch=master)](https://travis-ci.org/vlopes11/geomorph)
[![Latest version](https://img.shields.io/crates/v/geomorph.svg)](https://crates.io/crates/geomorph)
[![Documentation](https://docs.rs/geomorph/badge.svg)](https://docs.rs/geomorph)

Simple conversion between different coordinate systems without external wrappers injection

# Code Example
```
extern crate geomorph;
use geomorph::*;

fn try_main() -> Result<coord::Coord, ParseError> {
    let lat: f64 = -23.0095839;
    let lon: f64 = -43.4361816;
    
    coord::Coord::new(&lat, &lon)
}

fn try_main_utm(coord: coord::Coord) -> Result<utm::Utm, ParseError> {
    utm::Utm::new(&coord)
}

fn main() {
    let coord = try_main().unwrap();
    let utm = try_main_utm(coord);
}
```
