use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone)]
struct Range {
    min: isize,
    max: isize,
}

impl Range {
    fn new(min: isize, max: isize) -> Range {
        assert!(min <= max);
        Range { min, max }
    }

    // Returns the overlapping range, if any.
    fn intersect(&self, other: &Range) -> Option<Range> {
        if other.min <= self.max && self.min <= other.max {
            let intersection_min = self.min.max(other.min);
            let intersection_max = self.max.min(other.max);
            Some(Range::new(intersection_min, intersection_max))
        } else {
            None
        }
    }

    fn count(&self) -> isize {
        (self.max - self.min) + 1
    }
}

#[derive(Debug, Copy, Clone)]
struct Cuboid {
    on: bool,
    x: Range,
    y: Range,
    z: Range,
    score: isize,
}

impl Cuboid {
    // I don't want to talk about it.
    fn new(line: &String) -> Option<Cuboid> {
        if let Some((on, coords)) = line.split_once(" ") {
            if let Some((x, yz)) = coords.split_once(",") {
                if let Some((y, z)) = yz.split_once(",") {
                    if let Some((Ok(x_min), Ok(x_max))) = x.strip_prefix("x=").and_then(|s| {
                        s.split_once("..").and_then(|(start, end)| {
                            Some((start.parse::<isize>(), end.parse::<isize>()))
                        })
                    }) {
                        if let Some((Ok(y_min), Ok(y_max))) = y.strip_prefix("y=").and_then(|s| {
                            s.split_once("..").and_then(|(start, end)| {
                                Some((start.parse::<isize>(), end.parse::<isize>()))
                            })
                        }) {
                            if let Some((Ok(z_min), Ok(z_max))) =
                                z.strip_prefix("z=").and_then(|s| {
                                    s.split_once("..").and_then(|(start, end)| {
                                        Some((start.parse::<isize>(), end.parse::<isize>()))
                                    })
                                })
                            {
                                let on = on == "on";
                                let x = Range::new(x_min, x_max);
                                let y = Range::new(y_min, y_max);
                                let z = Range::new(z_min, z_max);
                                return Some(Cuboid {
                                    on,
                                    x,
                                    y,
                                    z,
                                    score: x.count() * y.count() * z.count(),
                                });
                            }
                        }
                    }
                }
            }
        }
        None
    }

    // If the other cuboid overlaps with us, return a new cuboid containing the
    // overlapping region
    fn overlap(&self, other: &Self) -> Option<Cuboid> {
        if let Some(x) = self.x.intersect(&other.x) {
            if let Some(y) = self.y.intersect(&other.y) {
                if let Some(z) = self.z.intersect(&other.z) {
                    return Some(Cuboid {
                        on: true,
                        x,
                        y,
                        z,
                        score: x.count() * y.count() * z.count(),
                    });
                }
            }
        }
        None
    }
}

fn parse(lines: Vec<String>) -> Result<Vec<Cuboid>, Box<dyn std::error::Error>> {
    Ok(lines
        .iter()
        .filter_map(|l| Cuboid::new(l))
        .collect::<Vec<Cuboid>>())
}

fn count_cubes(cuboids: &Vec<Cuboid>, limit: Option<Cuboid>) -> isize {
    let mut lights: Vec<Cuboid> = vec![];

    for cuboid in cuboids.iter() {
        if let Some(limit) = limit {
            if cuboid.overlap(&limit).is_none() {
                continue;
            }
        }
        let visited = lights.clone();
        if cuboid.on {
            lights.push(*cuboid);
        }
        for prev in visited {
            if let Some(mut new) = prev.overlap(&cuboid) {
                new.on = !prev.on;
                lights.push(new);
            }
        }
    }
    let on = lights
        .iter()
        .filter(|c| c.on)
        .map(|c| c.score)
        .sum::<isize>();
    let off = lights
        .iter()
        .filter(|c| !c.on)
        .map(|c| c.score)
        .sum::<isize>();

    on - off
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines = reader
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()?;

    let cuboids = parse(lines)?;

    let limit = Cuboid {
        on: true,
        x: Range::new(-50, 50),
        y: Range::new(-50, 50),
        z: Range::new(-50, 50),
        score: 0,
    };

    println!("Part 1: {}", count_cubes(&cuboids, Some(limit)));
    println!("Part 2: {}", count_cubes(&cuboids, None));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    const EXAMPLE2: &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn example1_overlap() -> Result<(), Box<dyn std::error::Error>> {
        let cuboids = parse(EXAMPLE1.lines().map(|l| l.to_string()).collect())?;

        assert!(count_cubes(&cuboids, None) == 39);

        Ok(())
    }

    #[test]
    fn example2_overlap() -> Result<(), Box<dyn std::error::Error>> {
        let cuboids = parse(EXAMPLE2.lines().map(|l| l.to_string()).collect())?;

        let limit = Cuboid {
            on: true,
            x: Range::new(-50, 50),
            y: Range::new(-50, 50),
            z: Range::new(-50, 50),
            score: 0,
        };

        assert!(count_cubes(&cuboids, Some(limit)) == 590784);

        Ok(())
    }
}
