use channel::Channel;
use color_space::{WhitePoint};
use num_traits::{Float, NumCast, cast, zero};
use xyz::{Xyz, ToXyz};
use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Lab<T, Wp>{
    pub l: T,
    pub a: T,
    pub b: T,
    pub white_point: Wp,
}

impl<T, Wp: WhitePoint> Lab<T, Wp>{
    pub fn new(l: T, a: T, b: T) -> Lab<T, Wp>{
        Lab { l, a, b, white_point: Wp::default() }
    }
}


impl<T: Copy, Wp: WhitePoint> Lab<T, Wp>{
    pub fn brightness(&self) -> T {
        self.l
    }
}

impl<T: Float, Wp: WhitePoint> Lab<T, Wp>{
    pub fn chromacity(&self) -> T {
        (self.a.powi(2) + self.b.powi(2)).sqrt()
    }

    pub fn hue(&self) -> T {
        let h = self.b.atan2(self.a);
        if h < zero() {
            h + cast(std::f64::consts::TAU).unwrap()
        }else{
            h
        }
    }

    pub fn offset_chromacity(&self, chroma_offset: T) -> Lab<T, Wp>{
        let current_croma = self.chromacity();
        let offset_a = self.a / current_croma * chroma_offset;
        let offset_b = self.b / current_croma * chroma_offset;
        Lab::new(
            self.l,
            self.a + offset_a,
            self.b + offset_b,
        )
    }
}

pub trait ToLab {
    type WhitePoint: WhitePoint;
    fn to_lab<T: Channel>(&self) -> Lab<T, Self::WhitePoint>;
}

impl<T: Channel + Float + NumCast, Wp: WhitePoint> ToXyz for Lab<T, Wp> {
    type WhitePoint = Wp;
    fn to_xyz<U: Channel + Float>(&self) -> Xyz<U, Wp> {
        let fy = (self.l + cast(16).unwrap()) / cast(116).unwrap();
        let fx = self.a / cast(500).unwrap() + fy;
        let fz = fy - self.b / cast(200).unwrap();

        let e = cast::<u32,T>(216).unwrap() / cast(24389).unwrap();
        let k = cast::<u32,T>(24389).unwrap() / cast(27).unwrap();
        let d = cast::<u32,T>(16).unwrap() / cast(116).unwrap();
        let fx3 = fx.powi(3);
        let fy3 = fy.powi(3);
        let fz3 = fz.powi(3);
        let xr = if fx3 > e {
            fx3
        }else{
            (fx - d) * k
        };
        let yr = if fy3 > e {
            fy3
        }else{
            (fy - d) * k
        };
        let zr = if fz3 > e {
            fz3
        }else{

impl<T: Channel + Float + NumCast, Wp: WhitePoint> Add for Lab<T,Wp>{
    type Output = Lab<T, Wp>;
    fn add(self, other: Lab<T, Wp>) -> Lab<T, Wp> {
        Lab::new(self.l + other.l, self.a + other.a, self.b + other.b)
    }
}

impl<T: Channel + Float + NumCast, Wp: WhitePoint> Mul<T> for Lab<T,Wp>{
    type Output = Lab<T, Wp>;
    fn mul(self, other: T) -> Lab<T, Wp> {
        Lab::new(self.l * other, self.a * other, self.b * other)
    }
}