mod tuples;
mod utils;
mod color;
mod canvas;
mod matrix;

use tuples::{Tuple};
use canvas::Canvas;
use color::Color;
use matrix::Matrix;
fn main() {
    println!("Hello, world!");

    let env = Environment {
        gravity: Tuple::new_vector(0., -0.1, 0.),
        wind: Tuple::new_vector(-0.01, 0., 0.)
    };

    let mut projectile = Projectile::new(Tuple::new_point(0., 1., 0.), Tuple::new_vector(1., 1.8, 0.).normalize() * 11.25);

    let mut canvas = Canvas::new(900, 550);
    let red = Color::white();

    loop {
        println!("{:?}", projectile);

        if projectile.position.y <= 0. {
            break;
        }

        canvas.write_pixel_at(projectile.position.x as usize, 550 - projectile.position.y as usize, red);
        projectile = tick(env, projectile);
    }

    //canvas.save_to_disk();
    
    let m = Matrix::construct_empty_3x3();
}

// Putting it Together CH 1
fn tick(env: Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile::new(position, velocity)
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Projectile {
    position: Tuple,
    velocity: Tuple
}

impl Projectile {
    fn new(position: Tuple, velocity: Tuple) -> Self {
        Projectile {
            position,
            velocity
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Environment {
    gravity: Tuple,
    wind: Tuple
}