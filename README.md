# Bouncy balls in Rust

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

![Image 1](/assets/img1.png)

Then I figured out how to draw a single ball:

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

Then I move it back and forth in the screen width:

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

Do the same for the `y` value:

```
use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    let w = screen_width();
    let h = screen_height();
    let diameter = 10.0;
    let mut x = 30.0;
    let mut x_add = 1.0;
    let mut y = 40.0;
    let mut y_add = 1.0;

    loop {
        clear_background(BLACK);

        draw_circle(x, y, diameter, RED);

        if x + diameter / 2.0 == w {
            x_add = -1.0;
        } else if x - diameter / 2.0 == 0.0 {
            x_add = 1.0;
        }

        if y + diameter / 2.0 == h {
            y_add = -1.0;
        } else if y - diameter / 2.0 == 0.0 {
            y_add = 1.0;
        }

        x += x_add;
        y += y_add;

        next_frame().await
    }
}
```

Create a `Ball` to encapsulate all the stuff for drawing and moving a single ball:

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

Now let's make lots of balls with random starting points and sizes and colors moving in different directions:

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

Make them disappear when two collide:

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

    fn collides(&self, other: &Ball) -> bool {
        let x_dist = (self.x - other.x).abs();
        let y_dist = (self.y - other.y).abs();
        let diameter = self.diameter.max(other.diameter);

        self.number != other.number
            && self.visible
            && other.visible
            && x_dist <= diameter
            && y_dist <= diameter
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
        remove.sort();
        for i in remove {
            balls[i].visible = false;
        }
        next_frame().await
    }
}
```

## Author

Ken Youens-Clark <kyclark@gmail.com>
