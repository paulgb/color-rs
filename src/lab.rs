use channel::Channel;
use color_space::{WhitePoint};
use num_traits::{Float, NumCast, cast};
use xyz::{Xyz, ToXyz};

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
            (fz - d) * k
        };
        let x = xr * Wp::xyz().x;
        let y = yr * Wp::xyz().y;
        let z = zr * Wp::xyz().z;

        Xyz::new(x.to_channel(), y.to_channel(), z.to_channel())
    }
}