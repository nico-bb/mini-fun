mod math;

use macroquad::prelude::*;

struct Workbench {
    components: Vec<Component>,
}

impl Workbench {
    fn add_component() {}

    fn update() {}

    fn draw(self: &Workbench) {
        for component in &self.components {
            component.draw();
        }
    }
}

struct Component {
    pos: math::Vec2,
    bounding_box: math::Rect,
    selected: bool,
    derived: AnyComponent,
}

enum AnyComponent {
    Empty(DebugComponent),
}

impl Component {
    fn select(x: f32, y: f32) {}

    fn draw(self: &Component) {
        match &self.derived {
            AnyComponent::Empty(c) => {
                draw_rectangle(
                    self.pos.x,
                    self.pos.y,
                    self.bounding_box.width,
                    self.bounding_box.height,
                    c.clr,
                );
            }
        }
    }
}

struct DebugComponent {
    clr: Color,
}

#[macroquad::main("HelloWorld")]
async fn main() {
    let clear_clr = Color {
        r: 0.55,
        g: 0.55,
        b: 0.55,
        a: 1.00,
    };

    let mut workbench = Workbench {
        components: Vec::new(),
    };
    workbench.components.push(Component {
        pos: math::Vec2 { x: 100.0, y: 100.0 },
        bounding_box: math::Rect {
            x: 100.0,
            y: 100.0,
            width: 100.0,
            height: 100.0,
        },
        selected: false,
        derived: AnyComponent::Empty(DebugComponent { clr: WHITE }),
    });
    loop {
        clear_background(clear_clr);

        workbench.draw();
        next_frame().await;
    }
}
