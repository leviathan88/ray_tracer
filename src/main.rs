mod tuples;
mod utils;

use tuples::{Tuple};

fn main() {
    println!("Hello, world!");

    let env = Environment {
        gravity: Tuple::new_vector(0., -0.1, 0.),
        wind: Tuple::new_vector(-0.01, 0., 0.)
    };

    let mut projectile = Projectile::new(Tuple::new_point(0., 1., 0.), Tuple::new_vector(1., 1., 0.).normalize());

    loop {
        println!("{:?}", projectile);

        if projectile.position.y <= 0. {
            break;
        }

        projectile = tick(env, projectile);
    }
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