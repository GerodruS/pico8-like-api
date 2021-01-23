# Pico8-like API above SDL2 on pure Rust language

**WIP**

## Setup

```toml
[dependencies]
pico8-like-api = { git = "https://github.com/GerodruS/pico8-like-api" }
```

## Template

```rust
use pico8_like_api::*;

struct MyGame {
}

impl MyGame {
    fn new() -> MyGame {
        MyGame {
        }
    }
}

impl Game for MyGame {
    fn init(&mut self, sys: &mut impl System) {
    }

    fn update(&mut self, sys: &mut impl System) {
    }

    fn draw(&mut self, sys: &mut impl System) {
    }
}

fn main() -> Result<(), String> {
    run(&mut MyGame::new())
}
```

## Examples

### Evening Summer Breeze

Source: https://twitter.com/von_rostock/status/1169000454396764161

```rust
use pico8_like_api::*;

struct MyGame {
    t: i32,
}

impl MyGame {
    fn new() -> MyGame {
        MyGame {
            t: 0,
        }
    }
}

impl Game for MyGame {
    fn draw(&mut self, sys: &mut impl System) {
        // source: https://twitter.com/von_rostock/status/1169000454396764161
        let r = [4,4,4,4,4,9,10,15,7];
        let o = 0.015;
        sys.cls(2);
        self.t += 1;
        sys.srand(0);
        for h in 0..=700 {
            let mut x = sys.rnd(136.0) - 4.0;
            let mut y = h as f32 / 5.0;
            let z = x + self.t as f32;
            let mut c = sys.sin(0.012*z+ sys.sin(o*y/2.0)/2.0) + sys.sin(o*y+ sys.sin(o*z/2.0)/4.0);
            let l = 1.0 + ((c + 2.0) * 1.5).floor();
            c /= 39.0;
            let mut b = 16.0;
            for i in 0..3  {
                let u=x+b* sys.sin(c);
                let v=y+b* sys.sin(c+0.25);
                b /= 2.0;
                c *= 2.0;
                let index = l as usize + i;
                let color = r[index];
                sys.line(x as u8, y as u8, u as u8, v as u8, color);
                x = u;
                y = v;
            }
        }
    }
}

fn main() -> Result<(), String> {
    run(&mut MyGame::new())
}
```

## Links

About Pico-8: https://www.lexaloffle.com/pico-8.php
