mod math;

use macroquad::prelude::*;

struct Workbench {
    components: Vec<Component>,
    selected_index: Option<usize>,
}

impl Workbench {
    fn add_component(self: &mut Workbench, component: Component) {
        self.components.push(component);
    }

    fn handle_input(self: &mut Workbench) {
        let (m_x, m_y) = mouse_position();
        let m_pos = math::Vec2 { x: m_x, y: m_y };

        if is_mouse_button_pressed(MouseButton::Left) {
            for (i, component) in self.components.iter_mut().enumerate() {
                if component.should_interact(&m_pos) {
                    component.select(m_pos);
                    self.selected_index = Some(i);
                }
            }
        } else if is_mouse_button_released(MouseButton::Left) {
            if let Some(index) = self.selected_index {
                self.components[index].deselect();
                self.selected_index = None;
            }
        }
    }

    fn update(self: &mut Workbench, dt: f32) {
        for (i, component) in self.components.iter_mut().enumerate() {
            component.update(dt);
        }
    }

    fn draw(self: &Workbench) {
        for component in &self.components {
            component.draw();
        }
    }
}

struct Component {
    pos: math::Vec2,
    bounding_box: math::Rect,
    offset: math::Vec2,
    selected: bool,
    derived: AnyComponent,
}

enum AnyComponent {
    Empty(DebugComponent),
}

impl Component {
    fn new(x: f32, y: f32, w: f32, h: f32, derived: AnyComponent) -> Component {
        return Component {
            pos: math::Vec2 { x: x, y: y },
            bounding_box: math::Rect {
                x: x,
                y: y,
                width: w,
                height: h,
            },
            offset: math::Vec2::zero(),
            selected: false,
            derived: derived,
        };
    }

    fn should_interact(self: &Component, m_pos: &math::Vec2) -> bool {
        match &self.derived {
            AnyComponent::Empty(c) => {
                return self.bounding_box.aabb_check(m_pos);
            }
        }
    }

    fn select(self: &mut Component, m_pos: math::Vec2) {
        self.selected = true;
        self.offset = m_pos.sub(self.pos);
    }

    fn deselect(self: &mut Component) {
        self.selected = false;
    }

    fn update(self: &mut Component, dt: f32) {
        if !self.selected {
            return;
        }

        // self
    }

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
        selected_index: None,
    };
    workbench.add_component(Component::new(
        100.0,
        100.0,
        100.0,
        100.0,
        AnyComponent::Empty(DebugComponent { clr: WHITE }),
    ));
    loop {
        workbench.handle_input();
        clear_background(clear_clr);

        workbench.draw();
        next_frame().await;
    }
}
