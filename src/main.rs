#![allow(non_snake_case)]
extern crate bmp;
use bmp::{Image, Pixel, px};

extern crate rand;
use rand::Rng;
use std::collections::HashMap;
use std::thread;

extern crate savefile;
use savefile::prelude::*;

use std::f64::consts::PI as PI;
const SIDES: usize = 7;
const SIZE: (u32, u32) = (1_000, 1_000);
const CENTRE: (u32, u32) = (SIZE.0/2, SIZE.1/2);
//const JUMP: f64 = SIDES as f64/(SIDES+3) as f64;
const JUMP: f64 = 0.5; 

fn polygon() -> [(u32, u32); SIDES] 
{
    let mut points = [(0, 0); SIDES];
    let interval = PI*2.0/(SIDES as f64);
    let mut angle: f64 = 0.0;
    for i in 0..SIDES {
        points[i] = (((CENTRE.0 as f64 *angle.cos()) as i64 +CENTRE.0 as i64) as u32, ((CENTRE.0 as f64 *angle.sin()) as i64 +CENTRE.1 as i64) as u32);
        angle += interval;
    }
    points
}

fn nextPoint(current: &(u32, u32), polygon: &[(u32, u32); SIDES]) -> (u32, u32)
{
    let mut rng = rand::thread_rng();
    //let mut i = old;
    //while i == old {
    let i = rng.gen_range(0..SIDES);
    //}
    let vertex = polygon[i];
    let lerpVertex: (u32, u32) = ((vertex.0 as f64*JUMP) as u32, (vertex.1 as f64*JUMP) as u32);
    let lerpCurrent: (u32, u32) = ((current.0 as f64*(1.0-JUMP)) as u32, (current.1 as f64*(1.0-JUMP)) as u32);
    //println!("{:?}, {:?}", lerpVertex, lerpCurrent);
    let point = (lerpVertex.0+lerpCurrent.0, lerpVertex.1+lerpCurrent.1);
    point
}

fn delve(current: &(u32, u32), polygon: &[(u32, u32); SIDES], counts: &mut HashMap<(u32, u32), u64>, mut encountered: Vec<(u32, u32)>, depth: u32) -> u64 {
    for point in &encountered {
        *counts.get_mut(&point).expect("New point outside image") += 1;
    }
    if !encountered.contains(current) {
        *counts.get_mut(current).expect("New point outside image") += 1;
        encountered.push(current.clone());
    }
    if depth == 0 {
        return 1;
    }
    let mut path_count = 0;
    for i in 0..SIDES {
        let next_point = nextPoint(current, polygon);
        path_count += delve(&next_point, polygon, counts, encountered.clone(), depth-1);
    }
    path_count
}
fn main() {
    if SIDES < 3 {
        panic!("Cannot have a polygon with less than 3 sides");
    }
    //for (x, y) in img.coordinates() {
    //    img.set_pixel(x, y, px![255, 255, 255])
    //}
    let polygon = polygon();
    //for vertex in polygon {
    //    for i in 0..10 {
    //        for j in 0..10 {
    //            img.set_pixel(min(vertex.0+i, 999), min(vertex.1+j, 999), px![0, 0, 0]);
    //        }
    //    }
    //}
    let mut point = CENTRE;
    //for j in 0..1_000_u64 {
    //    let pointCount = ((j as f64).powf(4_f64)*10_000_f64/(1_000_f64.powf(3_f64))) as u64;
        //println!("{}", pointCount);
    //    for _ in 0..pointCount {
    //        (i, point) = nextPoint(i, &point, &polygon);
    //        img.set_pixel(point.0, point.1, px![255, 255, 255]);
    //    }
    //let mut counts: HashMap::<(u32, u32), u64> = HashMap::with_capacity((SIZE.0*SIZE.1) as usize);
    //for x in 0..SIZE.0 {
    //    for y in 0..SIZE.1 {
    //        counts.insert((x, y), 0);
    //    }
    //}
    //for depth in 0..10 {
    //    let paths = delve(&CENTRE, &polygon, &mut counts, Vec::new(), depth);
    //    println!("{paths}");
    //    let max_count = counts.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    //    println!("max: {}", max_count.1);
    //    for (point, count) in &counts {
    //        if *count != 0 {
    //            //println!("prop: {count}");
    //        }
    //        let adj_count = count*(2u64.pow(8));
    //        let proportion = adj_count/paths;
    //        img.set_pixel(point.0, point.1, px![proportion, proportion, proportion]);
    //        img.set_pixel(point.0, point.1, px![proportion, proportion, proportion]);
    //    }
    //    let _ = img.save(&format!("images/{:0>4}.bmp", depth));
    //}
    //let mut new_points: HashMap::<(u32, u32), f64> = points.clone();
    //*points.get_mut(&CENTRE)=1f64;
    //for (point, value) in points {
    //    for i in 0..SIDES {
    //        new_point = nextPoint(&point, &polygon);
    //        *new_points.get_mut(&new_point)
    //    }
    //}
    let mut pointCounts = HashMap::with_capacity((SIZE.0*SIZE.1) as usize);
    for x in 0..SIZE.0 {
        for y in 0..SIZE.1 {
            pointCounts.insert((x, y), 0u32);
        }
    }
    for i in 0..1_000_000_000_000u64 {
        if i % 10_000_000 == 0 {
            println!("{}/100_000", i/10_000_000);
        }
        *pointCounts.get_mut(&point).unwrap() += 1;
        point = nextPoint(&point, &polygon);
    }
    save_file("hept.data", 0, &pointCounts);
    //return;
    pointCounts = load_file("hept.data", 0).unwrap();
    let max: u32 = *pointCounts
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap().1;
    std::thread::scope(|s| {
        let pointCounts = &pointCounts;
        for i in 0..max {
            s.spawn( move || {
                let mut img = Image::new(SIZE.0, SIZE.1);
                for (point, count) in pointCounts {
                    if *count > i {
                        img.set_pixel(point.0, point.1, px!(255, 255, 255));
                    }
                }
                let _ = img.save(&format!("images/{:0>8}.bmp", i));
            });
        }
    });
}
