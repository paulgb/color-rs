// Copyright 2013 The color-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use channel::Channel;
use num_traits::Float;
use rgb::{Rgb, ToRgb};
use yxy::{Yxy, ToYxy};
use color_space::{D65, WhitePoint, MatrixColorSpace, Srgb, TransferFunction};
use num_traits::{zero, NumCast, cast};
// use srgb::{Srgb, ToSrgb};
use lab::{Lab, ToLab};

#[derive(Clone, Copy, Debug)]
pub struct Xyz<T = f32, Wp = D65>
where T: Channel + Float
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub white_point: Wp,
}

impl<T: Channel + Float, Wp: WhitePoint> Xyz<T,Wp> {
    pub fn new(x: T, y: T, z: T) -> Xyz<T,Wp> {
        Xyz{
            x,
            y,
            z,
            white_point: Wp::default(),
        }
    }
}

pub trait ToXyz {
    type WhitePoint: WhitePoint;
    fn to_xyz<T: Channel + Float + std::fmt::Debug>(&self) -> Xyz<T, Self::WhitePoint>;
}

impl<T: Channel + Float + Clone> ToRgb for Xyz<T, D65> {
    type Standard = Srgb;
    fn to_rgb<U: Channel>(&self) -> Rgb<U, Srgb> {
        let rgb = Srgb::to_rgb_matrix() * self.clone().into();
        Rgb::new(
            Srgb::from_linear(rgb[0]).to_channel(),
            Srgb::from_linear(rgb[1]).to_channel(),
            Srgb::from_linear(rgb[2]).to_channel(),
        )
    }
}

impl<T: Channel + Float + NumCast, Wp: WhitePoint> ToLab for Xyz<T, Wp> {
    type WhitePoint = Wp;
    fn to_lab<U:Channel>(&self) -> Lab<U, Wp> {
        let xr = self.x / Wp::xyz().x;
        let yr = self.y / Wp::xyz().y;
        let zr = self.z / Wp::xyz().z;
        let e: T = cast::<u32, T>(216).unwrap() / cast(24389).unwrap();
        let k: T = cast::<u32, T>(24389).unwrap() / cast(27).unwrap();
        let d: T = cast::<u32, T>(16).unwrap() / cast(116).unwrap();
        let fx = if xr > e {
            xr.cbrt()
        }else{
            k * xr + d
        };
        let fy = if yr > e {
            yr.cbrt()
        }else{
            k * yr + d
        };
        let fz = if zr > e {
            zr.cbrt()
        }else{
            k * zr + d
        };
        let l: T = cast::<u32, T>(116).unwrap() * fy - cast(16).unwrap();
        let a: T = cast::<u32, T>(500).unwrap() * (fx - fy);
        let b: T = cast::<u32, T>(200).unwrap() * (fy - fz);
        Lab{l: l.to_channel(), a: a.to_channel(), b: b.to_channel(), white_point: Wp::default()}
    }
}

impl<T: Channel + Float, Wp: WhitePoint> ToYxy for Xyz<T, Wp> {
    type WhitePoint = Wp;
    fn to_yxy<U: Channel + Float>(&self) -> Yxy<U, Wp> {
        let sum = self.x + self.y + self.z;
        let x;
        let y;
        let luma = self.y;
        if sum < zero() || sum > zero() {
            x = self.x / sum;
            y = self.y / sum;
        }else{
            x = zero();
            y = zero();
        }
        Yxy{x: x.to_channel(), y: y.to_channel(), luma: luma.to_channel(), white_point: Wp::default()}
    }
}