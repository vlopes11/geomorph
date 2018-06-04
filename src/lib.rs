pub struct Coord {
    lat: f64,
    lon: f64,
}

pub struct Utm {
    zone: u32,
    north: bool,
    x: f64,
    y: f64,
}

impl Utm {
    pub fn new(coord: Coord) -> Utm {
        let lat = coord.lat;
        let lon = coord.lon;

        let north = lat >= 0.0;
        let mut zone = 0;
        let x;
        let y;
    
        x = 444159.6758494562;
        y = 3688032.3998262062;
        
        // Set zone
        if lat >= -80.0 && lat < 84.0 {
            let fmod_lon = lon - 360.0 * (lon / 360.0).trunc();
            let floor_lon = fmod_lon.floor();

            let ilon;
            if floor_lon >= 180.0 {ilon = floor_lon - 360.0;}
            else if floor_lon < -180.0 {ilon = floor_lon + 360.0;}
            else {ilon = floor_lon;}

            zone = ((ilon + 186.0) / 6.0).trunc() as u32;

            let band =
                ((lat.floor() + 80.0) / 8.0 - 10.0)
                    .min(9.0)
                    .max(-10.0)
                    .trunc();
            if band == 7.0 && zone == 31 && ilon >= 3.0 {
                zone = 32;    // Norway UTM exception
            } else if band == 9.0 && ilon >= 0.0 && ilon < 42.0 {
                // Svalbard UTM exception
                zone =
                    (2.0 * ((ilon + 183.0) / 12.0).trunc() + 1.0) as u32;
            }
        } else {
            zone = 0;
        }

        Utm {
            zone,
            north,
            x,
            y,
        }
    }

    pub fn to_string(&self) -> String {
        let mut utm_str = self.zone.to_string();
        if self.north {
            utm_str.push_str("n ");
        } else {
            utm_str.push_str("s ");
        }
        utm_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utm_zone_south() {
        let coord = Coord {lat: -23.0095839, lon: -43.4361816};
        let utm = Utm::new(coord);
        assert_eq!(utm.zone, 23);
        assert_eq!(utm.north, false);
    }

    #[test]
    fn utm_norway_zone() {
        let coord = Coord {lat: 61.076521, lon: 4.680180};
        let utm = Utm::new(coord);
        assert_eq!(utm.zone, 32);
        assert_eq!(utm.north, true);
    }

    #[test]
    fn utm_svalbard_zone() {
        assert_eq!(Utm::new(Coord {
            lat: 78.891608,
            lon: 10.457194,
        }).zone, 33);
        assert_eq!(Utm::new(Coord {
            lat: 78.122200,
            lon: 20.349504,
        }).zone, 33);
        assert_eq!(Utm::new(Coord {
            lat: 78.102575,
            lon: 21.013745,
        }).zone, 35);
        assert_eq!(Utm::new(Coord {
            lat: 78.138264,
            lon: 30.194746,
        }).zone, 35);
    }
}
