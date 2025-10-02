use std::collections::HashMap;
use std::num::ParseIntError;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    const fn size(self) -> u64 {
        self.x.unsigned_abs() + self.y.unsigned_abs() + self.z.unsigned_abs()
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl Mul<i64> for Vector {
    type Output = Self;

    fn mul(mut self, scale: i64) -> Self::Output {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
        self
    }
}

impl Div<i64> for Vector {
    type Output = Self;

    fn div(mut self, scale: i64) -> Self::Output {
        self.x /= scale;
        self.y /= scale;
        self.z /= scale;
        self
    }
}

impl FromStr for Vector {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts
            .next()
            .ok_or(ParseError::SyntaxError)?
            .trim_ascii()
            .parse()?;
        let y = parts
            .next()
            .ok_or(ParseError::SyntaxError)?
            .trim_ascii()
            .parse()?;
        let z = parts
            .next()
            .ok_or(ParseError::SyntaxError)?
            .trim_ascii()
            .parse()?;
        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Particle {
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
}

impl Particle {
    fn tick(&mut self) {
        self.velocity = self.velocity + self.acceleration;
        self.position = self.position + self.velocity;
    }
}

impl FromStr for Particle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s.strip_prefix("p=<").ok_or(ParseError::SyntaxError)?;
        let (position, rest) = rest.split_once(">, v=<").ok_or(ParseError::SyntaxError)?;
        let position = position.parse()?;
        let (velocity, rest) = rest.split_once(">, a=<").ok_or(ParseError::SyntaxError)?;
        let velocity = velocity.parse()?;
        let acceleration = rest.strip_suffix('>').ok_or(ParseError::SyntaxError)?;
        let acceleration = acceleration.parse()?;
        Ok(Self {
            position,
            velocity,
            acceleration,
        })
    }
}

impl Sub for Particle {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.position = self.position - rhs.position;
        self.velocity = self.velocity - rhs.velocity;
        self.acceleration = self.acceleration - rhs.acceleration;
        self
    }
}

#[aoc_generator(day20)]
fn parse(input: &str) -> Result<Vec<Particle>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day20, part1)]
fn part_1(particles: &[Particle]) -> usize {
    let min_acceleration = particles
        .iter()
        .map(|p| p.acceleration.size())
        .min()
        .unwrap();
    let min_velocity = particles
        .iter()
        .filter(|p| p.acceleration.size() == min_acceleration)
        .map(|p| p.velocity.size())
        .min()
        .unwrap();
    particles
        .iter()
        .position(|p| {
            p.acceleration.size() == min_acceleration && p.velocity.size() == min_velocity
        })
        .unwrap()
}

#[aoc(day20, part2)]
fn part_2(particles: &[Particle]) -> usize {
    let mut particles = particles.to_vec();
    let mut counts = HashMap::<Vector, usize>::new();
    for _ in 1..100 {
        counts.clear();
        for particle in &mut particles {
            particle.tick();
            *counts.entry(particle.position).or_default() += 1;
        }
        particles.retain(|p| counts[&p.position] == 1);
    }
    particles.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
        p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>\n\
        p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>\
    ";
    const EXAMPLE2: &str = "\
        p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>\n\
        p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>\n\
        p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>\n\
        p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>\
    ";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE2).unwrap();
        assert_eq!(
            result,
            [
                Particle {
                    position: Vector { x: -6, y: 0, z: 0 },
                    velocity: Vector { x: 3, y: 0, z: 0 },
                    acceleration: Vector { x: 0, y: 0, z: 0 }
                },
                Particle {
                    position: Vector { x: -4, y: 0, z: 0 },
                    velocity: Vector { x: 2, y: 0, z: 0 },
                    acceleration: Vector { x: 0, y: 0, z: 0 }
                },
                Particle {
                    position: Vector { x: -2, y: 0, z: 0 },
                    velocity: Vector { x: 1, y: 0, z: 0 },
                    acceleration: Vector { x: 0, y: 0, z: 0 }
                },
                Particle {
                    position: Vector { x: 3, y: 0, z: 0 },
                    velocity: Vector { x: -1, y: 0, z: 0 },
                    acceleration: Vector { x: 0, y: 0, z: 0 }
                },
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let particles = parse(EXAMPLE1).unwrap();
        let result = part_1(&particles);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_2() {
        let particles = parse(EXAMPLE2).unwrap();
        let result = part_2(&particles);
        assert_eq!(result, 1);
    }
}
