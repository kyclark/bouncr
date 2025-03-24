# Bouncy balls in Rust

How to make a basic animation of bouncing balls using Rust and Macroquad.

## Setup

Follow the instructions at https://www.rust-lang.org/tools/install to install the Rust programming language.

Then:

```
git clone https://github.com/kyclark/bouncr.git
cd bouncr
cargo run
```

## Explanation

I started with the example from https://macroquad.rs/:

```
use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
```

This produces what appears to be a static image, but in truth the same image is being drawn to the screen repeatedly:

![Image 1](/assets/img1.png)

My goal is to make a bunch of balls bounce around the screen, so I first figured out how to draw a single ball.
I found the [https://docs.rs/macroquad/latest/macroquad/shapes/fn.draw_circle.html](draw_circle) function that is included with `macroquad::prelude`:

```
use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_circle(30.0, 40.0, 10.0, RED);

        next_frame().await
    }
}
```

As before, this draws the same circle to the same place on the screen over and again:


![Image 2](/assets/img2.png)

Next, I find want to move the circle back and forth across the screen.
I store the [https://docs.rs/macroquad/latest/macroquad/window/fn.screen_width.html](screen_width) and create two _mutable_ variables for the current `x` position and the `x_add` value that I'll add on each frame.
This will start as a positive value and will turn negative when the ball hits the right side of the screen (`screen_width`) and positive again when it hits the left side (`0`):

```
use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    let w = screen_width();
    let diameter = 10.0;
    let mut x = 30.0;
    let mut x_add = 1.0;

    loop {
        clear_background(BLACK);

        draw_circle(x, 40.0, diameter, RED);

        if x + diameter / 2.0 == w {
            x_add = -1.0;
        } else if x - diameter / 2.0 == 0.0 {
            x_add = 1.0;
        }

        x += x_add;

        next_frame().await
    }
}
```

Here's what that looks like:

![Movie 1](/assets/mov1.gif)

Next, I do the same for the `y` value using the [https://docs.rs/macroquad/latest/macroquad/window/fn.screen_height.html](screen_height):

```
use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    let w = screen_width();
    let h = screen_height();
    let diameter = 10.0;
    let radius = diameter / 2.0;
    let mut x = 30.0;
    let mut x_add = 1.0;
    let mut y = 40.0;
    let mut y_add = 1.0;

    loop {
        clear_background(BLACK);

        draw_circle(x, y, diameter, RED);

        if x + radius == w || x - radius == 0.0 {
            x_add *= -1.0;
        }

        if y + radius == h || y - radius == 0.0 {
            y_add *= -1.0;
        }

        x += x_add;
        y += y_add;

        next_frame().await
    }
}
```

I want to have lots of balls, so I need some way to represent the idea of one of them.
I can create a Rust `struct` called a `Ball` to encapsulate all the stuff for drawing and moving a single ball.
The `impl` (implementation) block has the functions:
* `new`: create a ball at some position with some diameter
* `render`: draw the ball on the screen
* `shift`: move the ball

The `main` function will still create just one ball.
The behavior is the same as before, but now the `main` function is much smaller because all the complexity is hidden inside the `Ball`:

```
use macroquad::prelude::*;

#[derive(Debug)]
struct Ball {
    x: f32,
    y: f32,
    diameter: f32,
    radius: f32,
    x_add: f32,
    y_add: f32,
    max_x: f32,
    max_y: f32,
}

impl Ball {
    fn new(x: f32, y: f32, width: f32, height: f32, diameter: f32) -> Self {
        Ball {
            x,
            y,
            diameter,
            radius: diameter / 2.0,
            x_add: 1.0,
            y_add: 1.0,
            max_x: width,
            max_y: height,
        }
    }

    fn render(&self) {
        draw_circle(self.x, self.y, self.diameter, RED);
    }

    fn shift(&mut self) {
        if self.x + self.radius == self.max_x || self.x - self.radius <= 0.0 {
            self.x_add *= -1.0;
        }

        if self.y + self.radius == self.max_y || self.y - self.radius == 0.0 {
            self.y_add *= -1.0;
        }

        self.x += self.x_add;
        self.y += self.y_add;
    }
}

#[macroquad::main("Bouncr")]
async fn main() {
    let w = screen_width();
    let h = screen_height();
    let mut ball = Ball::new(30.0, 40.0, w, h, 10.0);

    loop {
        clear_background(BLACK);
        ball.render();
        ball.shift();
        next_frame().await
    }
}
```

Now I can make lots of balls with random starting points and sizes and colors moving in different directions.
The `Ball` gets a little more complicated as I need to accept the ball's number along with a random number generator (RNG) for getting random values for `x`/`y` and diameter and color values.
The `rand` module conflicts with some of the exports from `macroquad`, so I'll be more explicit about what I want to import:

```
use macroquad::{
    color::{self, colors::*},
    shapes::*,
    window::*,
};
use rand::Rng;

#[derive(Debug)]
struct Ball {
    x: f32,
    y: f32,
    diameter: f32,
    radius: f32,
    x_add: f32,
    y_add: f32,
    max_x: f32,
    max_y: f32,
    color: color::Color,
}

impl Ball {
    fn new(rng: &mut impl Rng, max_x: f32, max_y: f32) -> Self {
        let diameter = rng.random_range(3.0..10.0);
        Ball {
            x: rng.random_range(0.0..max_x),
            y: rng.random_range(0.0..max_y),
            diameter,
            radius: diameter / 2.0,
            x_add: if rng.random_bool(0.5) { 1.0 } else { -1.0 },
            y_add: if rng.random_bool(0.5) { 1.0 } else { -1.0 },
            max_x,
            max_y,
            color: color::Color {
                r: rng.random_range(0.0..1.0),
                g: rng.random_range(0.0..1.0),
                b: rng.random_range(0.0..1.0),
                a: 1.0,
            },
        }
    }

    fn render(&self) {
        draw_circle(self.x, self.y, self.diameter, self.color);
    }

    fn shift(&mut self) {
        if self.x + self.radius >= self.max_x || self.x - self.radius <= 0.0 {
            self.x_add *= -1.0;
        }

        if self.y + self.radius >= self.max_y || self.y - self.radius <= 0.0 {
            self.y_add *= -1.0;
        }

        self.x += self.x_add;
        self.y += self.y_add;
    }
}

#[macroquad::main("Bouncr")]
async fn main() {
    let w = screen_width();
    let h = screen_height();
    let mut rng = rand::rng();
    let num_balls: usize = rng.random_range(10..100);
    let mut balls: Vec<Ball> =
        (0..num_balls).map(|_| Ball::new(&mut rng, w, h)).collect();

    loop {
        clear_background(BLACK);
        for ball in &mut balls {
            ball.render();
            ball.shift();
        }
        next_frame().await
    }
}
```

I thought it would be interesting to have a collision between two balls result in their disappearance or having them bounce off each other:

```
use macroquad::{
    color::{self, colors::*},
    shapes::*,
    window::*,
};
use rand::Rng;

#[derive(Debug)]
struct Ball {
    number: usize,
    x: f32,
    y: f32,
    diameter: f32,
    radius: f32,
    x_add: f32,
    y_add: f32,
    max_x: f32,
    max_y: f32,
    color: color::Color,
    visible: bool,
}

impl Ball {
    fn new(
        number: usize,
        rng: &mut impl Rng,
        max_x: f32,
        max_y: f32,
    ) -> Self {
        let diameter = rng.random_range(3.0..10.0);
        Ball {
            number,
            x: rng.random_range(0.0..max_x),
            y: rng.random_range(0.0..max_y),
            diameter,
            radius: diameter / 2.0,
            x_add: if rng.random_bool(0.5) { 1.0 } else { -1.0 },
            y_add: if rng.random_bool(0.5) { 1.0 } else { -1.0 },
            max_x,
            max_y,
            color: color::Color {
                r: rng.random_range(0.0..1.0),
                g: rng.random_range(0.0..1.0),
                b: rng.random_range(0.0..1.0),
                a: 1.0,
            },
            visible: true,
        }
    }

    fn render(&self) {
        if self.visible {
            draw_circle(self.x, self.y, self.diameter, self.color);
        }
    }

    fn shift(&mut self) {
        if self.x + self.radius >= self.max_x || self.x - self.radius <= 0.0 {
            self.x_add *= -1.0;
        }

        if self.y + self.radius >= self.max_y || self.y - self.radius <= 0.0 {
            self.y_add *= -1.0;
        }

        self.x += self.x_add;
        self.y += self.y_add;
    }

    fn reverse(&mut self) {
        self.x_add *= -1.0;
        self.y_add *= -1.0;
    }

    fn collides(&self, other: &Ball) -> bool {
        let x_dist = (self.x - other.x).abs();
        let y_dist = (self.y - other.y).abs();
        let dist = (x_dist.powi(2) + y_dist.powi(2)).sqrt();
        let radii = self.radius + other.radius;

        self.number != other.number
            && self.visible
            && other.visible
            && dist <= radii
    }
}

#[macroquad::main("Bouncr")]
async fn main() {
    let w = screen_width();
    let h = screen_height();
    let mut rng = rand::rng();
    let num_balls: usize = rng.random_range(10..100);
    let mut balls: Vec<Ball> = (0..num_balls)
        .map(|number| Ball::new(number, &mut rng, w, h))
        .collect();

    loop {
        clear_background(BLACK);
        for ball in &mut balls {
            ball.render();
            ball.shift();
        }
        let mut remove = vec![];
        for i in 0..num_balls {
            let ball = &balls[i];
            for j in i..num_balls {
                let other = &balls[j];
                if ball.collides(&other) {
                    remove.push(i);
                    remove.push(j);
                }
            }
        }
        for i in remove {
            // Either make the balls disappear
            // balls[i].visible = false;

            // Or have them bounce off each other
            balls[i].reverse();
        }
        next_frame().await
    }
}
```

## Author

Ken Youens-Clark <kyclark@gmail.com>
