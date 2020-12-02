extern crate regex;

use std::fs;
use regex::Regex;

fn main() {
    // read input & split into lines
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let lines = contents.lines();

    let claims: Vec<Claim> = lines.map(|l| parse_claim(l)).collect();

    let grid = make_grid(&claims);

    let part1 = grid.v.iter().filter(|&x| (*x) > 1).count();
    println!("part 1: {}", part1);

    let part2: Vec<&str> = claims.iter().filter(|c| non_overlapping(&c, &grid)).map(|c| c.id).collect();
    println!("part 2: {:?}", part2);
}

#[derive(Debug)]
struct Claim<'a> {
    id: &'a str,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl<'a> Claim<'a> {
    fn r(&self) -> u32 {
        return self.x + self.w;
    }

    fn b(&self) -> u32 {
        return self.y + self.h;
    }
}

fn parse_claim(line: &str) -> Claim {
    // "#35 @ 285,967: 23x28"
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").expect("failed to match regexp");
    let m = re.captures(line).expect("failed to unpack re captures");

    let id = m.get(1).map_or("", |m| m.as_str());
    let x: u32 = m.get(2).map_or("", |m| m.as_str()).parse().expect("failed to convert x to int");
    let y: u32 = m.get(3).map_or("", |m| m.as_str()).parse().expect("failed to convert y to int");
    let w: u32 = m.get(4).map_or("", |m| m.as_str()).parse().expect("failed to convert w to int");
    let h: u32 = m.get(5).map_or("", |m| m.as_str()).parse().expect("failed to convert h to int");

    return Claim {
        id: id,
        x: x,
        y: y,
        w: w,
        h: h,
    }
}

#[derive(Debug)]
struct Grid {
    w: u32,
    h: u32,
    v: Vec<u8>,
}

fn make_grid(claims: &[Claim]) -> Grid {
    let vw = claims.iter().map(|c| c.r()).max().expect("failed to get right boundary");
    let vh = claims.iter().map(|c| c.b()).max().expect("failed to get bottom boundary");
    let vs: usize = (vw * vh) as usize;

    println!("vw: {:?}", vw);
    println!("vh: {:?}", vh);
    println!("vs: {:?}", vs);

    let mut v: Vec<u8> = vec![0; vs];

    for claim in claims {
        //println!("boop: {:?} {} {}", claim, claim.r(), claim.b());
        for dy in claim.y..claim.b() {
            for dx in claim.x..claim.r() {
                let i = ((dy * vw) + dx) as usize;
                //println!("foo: {:?} {:?} {:?} {:?}", claim, dx, dy, i);
                v[i] += 1;
            }
        }
    }

    return Grid {
        w: vw,
        h: vh,
        v: v,
    };
}

fn non_overlapping(claim: &Claim, grid: &Grid) -> bool {
    for dy in claim.y..claim.b() {
        for dx in claim.x..claim.r() {
            let i = ((dy * grid.w) + dx) as usize;
            //println!("foo: {:?} {:?} {:?} {:?} {:?}", claim, dx, dy, i, grid.v[i]);
            if grid.v[i] > 1 {
                return false;
            }
        }
    }

    return true;
}
