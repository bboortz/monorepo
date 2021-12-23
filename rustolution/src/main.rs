use bracket_lib::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt;

const HEIGHT: u16 = 100;
const WIDTH: u16 = 100;

struct Position {
    x: u16,
    y: u16,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pos:({},{})", self.x, self.y)
    }
}

impl Distribution<Position> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Position {
        let rand_x = rng.gen_range(0..WIDTH);
        let rand_y = rng.gen_range(0..HEIGHT);
        Position {
            x: rand_x,
            y: rand_y,
        }
    }
}

struct Color {
    red: u16,
    green: u16,
    blue: u16,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color:({:x},{:x},{:x})", self.red, self.green, self.blue)
    }
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        let rand_red = rng.gen_range(0..255);
        let rand_green = rng.gen_range(0..255);
        let rand_blue = rng.gen_range(0..255);
        Color {
            red: rand_red,
            green: rand_green,
            blue: rand_blue,
        }
    }
}

struct Size {
    height: u16,
    width: u16,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Size:({},{})", self.height, self.width)
    }
}

impl Default for Size {
    fn default() -> Self {
        Size {
            height: 100,
            width: 100,
        }
    }
}

impl Distribution<Size> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Size {
        let rand_height = rng.gen_range(1..5);
        let rand_width = rng.gen_range(1..5);
        Size {
            height: rand_height,
            width: rand_width,
        }
    }
}

struct Age {
    age: u16,
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Age:({})", self.age)
    }
}

impl Default for Age {
    fn default() -> Self {
        Age { age: 0 }
    }
}

////

struct Creature {
    pos: Position,
    color: Color,
    size: Size,
    age: Age,
}

impl fmt::Display for Creature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - Creature:({}, {}, {})",
            self.pos, self.color, self.size, self.age
        )
    }
}

impl Distribution<Creature> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Creature {
        let pos: Position = rng.gen();
        let color: Color = rng.gen();
        let size: Size = rng.gen();
        let age = Age { age: 0 };
        Creature {
            pos: pos,
            color: color,
            size: size,
            age: age,
        }
    }
}

trait Living {
    fn new(pos: Position, color: Color, size: Size) -> Self;
    fn olden(&mut self);
}

impl Living for Creature {
    fn new(pos: Position, color: Color, size: Size) -> Creature {
        let age = Age { age: 0 };
        Creature {
            pos: pos,
            color: color,
            size: size,
            age: age,
        }
    }

    fn olden(&mut self) {
        self.age.age += 1;
    }
}

////

struct CreatureFactory {}

trait Factory {
    fn create(&self) -> Creature;
}

impl Factory for CreatureFactory {
    fn create(&self) -> Creature {
        let mut rng = rand::thread_rng();
        let c: Creature = rng.gen();
        c
    }
}

////

#[derive(Default)]
struct World {
    size: Size,
    age: Age,
    creature_vec: Vec<Creature>,
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "World:({}, {}, Len:{})",
            self.size,
            self.age,
            self.creature_vec.len()
        )
    }
}

trait Habitable {
    fn new(size: Size) -> Self;
    fn olden(&mut self);
    fn add_creature(&mut self, creature: Creature);
    fn print_creatures(&self);
}

impl Habitable for World {
    fn new(size: Size) -> World {
        let age = Age { age: 0 };
        World {
            size: size,
            age: age,
            creature_vec: Vec::new(),
        }
    }

    fn olden(&mut self) {
        self.age.age += 1;
        for c in self.creature_vec.iter_mut() {
            c.olden();
        }
    }

    fn add_creature(&mut self, creature: Creature) {
        self.creature_vec.push(creature);
    }

    fn print_creatures(&self) {
        for c in &self.creature_vec {
            println!("{}", c);
        }
    }
}

/////

struct State {
    world: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        /*
                let col1 = RGB::named(CYAN);
                let col2 = RGB::named(YELLOW);
                let percent: f32 = self.y as f32 / 50.0;
                let fg = col1.lerp(col2, percent);
        */
        let fg = RGB::named(WHITE);
        let bg = RGB::named(BLACK);

        ctx.cls();
        for c in self.world.creature_vec.iter_mut() {
            ctx.print_color(c.pos.x, c.pos.y, fg, bg, "@");
        }

        /*
        for x in 0..self.world.size.width {
            ctx.print_color(x, 0, fg, bg, x);
        }
        for y in 0..self.world.size.height {
            ctx.print_color(0, y, fg, bg, y);
        }
                */
        /*
                ctx.printer(
                    40,
                    49,
                    "#[blue]Hello #[pink]Bracket#[] world.",
                    TextAlign::Center,
                    Some(RGBA::from_u8(200, 200, 200, 255)),
                );

                ctx.print_color(15, self.y, fg, RGB::named(BLACK), self.y);


                if self.going_down {
                    self.y += 1;
                    if self.y > 48 {
                        self.going_down = false;
                    }
                } else {
                    self.y -= 1;
                    if self.y < 2 {
                        self.going_down = true;
                    }
                }

                ctx.draw_box(39, 0, 20, 3, RGB::named(WHITE), RGB::named(BLACK));
                ctx.printer(
                    58,
                    1,
                    &format!("#[pink]FPS: #[]{}", ctx.fps),
                    TextAlign::Right,
                    None,
                );
                ctx.printer(
                    58,
                    2,
                    &format!("#[pink]Frame Time: #[]{} ms", ctx.frame_time_ms),
                    TextAlign::Right,
                    None,
                );
        */
    }
}

///

fn main() -> BError {
    let world_size = Size {
        height: HEIGHT,
        width: WIDTH,
    };

    let mut w = World::new(world_size);
    let cf = CreatureFactory {};

    for _ in 1..2000 {
        w.add_creature(cf.create());
    }

    println!("{}", w);
    w.print_creatures();

    let context = BTermBuilder::simple(w.size.width, w.size.height)
        .unwrap()
        .with_title("rustolution")
        .with_automatic_console_resize(true)
        .with_fps_cap(120.0)
        .build()?;

    let gs: State = State { world: w };

    /*
        register_palette_color("blue", RGB::named(BLUE));
        register_palette_color("pink", RGB::named(MAGENTA));
    */

    main_loop(context, gs)
}

/*
fn main() {


    let color = Color {
        red: 0,
        green: 0,
        blue: 0,
    };
    let size = Size {
        height: 1,
        width: 1,
    };
    c = Creature::new(color, size);
    w.add_creature(c);
    println!("{}", w);
    w.print_creatures();

    w.olden();
    println!("{}", w);
    w.print_creatures();

    let color = Color {
        red: 1,
        green: 1,
        blue: 1,
    };
    let size = Size {
        height: 1,
        width: 1,
    };
    c = Creature::new(color, size);
    w.add_creature(c);

    w.olden();
    println!("{}", w);
    w.print_creatures();
}
*/
