mod interpreter;
mod math;

use interpreter::Value;
use macroquad::prelude::*;

struct Workbench {
    components: Vec<Component>,
    selected_index: Option<usize>,
    m_pos: math::Vec2,
    previous_m_pos: math::Vec2,
    m_delta: math::Vec2,

    cell_size: i32,
    theme: Theme,
}

struct Theme {
    background_clr: Color,
    separator_clr: Color,
}

impl Workbench {
    fn new() -> Workbench {
        return Workbench {
            components: Vec::new(),
            selected_index: None,
            m_pos: math::Vec2::zero(),
            previous_m_pos: math::Vec2::zero(),
            m_delta: math::Vec2::zero(),
            cell_size: 32,
            theme: Theme {
                background_clr: Color {
                    r: 28.0 / 255.0,
                    g: 32.0 / 255.0,
                    b: 33.0 / 255.0,
                    a: 1.0,
                },
                separator_clr: Color {
                    r: 61.0 / 255.0,
                    g: 56.0 / 255.0,
                    b: 54.0 / 255.0,
                    a: 1.0,
                },
            },
        };
    }

    fn add_component(self: &mut Workbench, component: Component) {
        self.components.push(component);
    }

    fn handle_input(self: &mut Workbench) {
        let (m_x, m_y) = mouse_position();
        self.previous_m_pos = self.m_pos;
        self.m_pos = math::Vec2 { x: m_x, y: m_y };
        self.m_delta = self.m_pos.sub(self.previous_m_pos);

        if is_mouse_button_pressed(MouseButton::Left) {
            for (i, component) in self.components.iter_mut().enumerate() {
                if component.should_interact(&self.m_pos) {
                    component.select(self.m_pos);
                    self.selected_index = Some(i);
                }
            }
        }
        if is_mouse_button_released(MouseButton::Left) {
            if let Some(index) = self.selected_index {
                self.components[index].deselect();
                self.selected_index = None;
            }
        }
    }

    fn update(self: &mut Workbench, dt: f32) {
        for component in self.components.iter_mut() {
            component.update(dt, self.m_pos, self.cell_size);
        }
    }

    fn draw(self: &Workbench) {
        for component in &self.components {
            component.draw(&self.theme);
        }

        const MARGIN: f32 = 15.0;
        let rect = math::Rect {
            x: MARGIN,
            y: MARGIN,
            width: screen_width() - MARGIN * 2.0,
            height: screen_height() - MARGIN * 2.0,
        };
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            4.0,
            self.theme.separator_clr,
        );
    }
}

struct Component {
    pos: math::Vec2,
    bounding_box: math::Rect,
    offset: math::Vec2,
    clr: Color,
    selected: bool,
    derived: AnyComponent,
}

enum AnyComponent {
    Empty,
    LogicGate {
        chunk: interpreter::Chunk,
        input_pins: [Box<Component>; 2],
        output_pin: Box<Component>,
    },
    Pin(PinComponent),
}

impl Component {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        return Component {
            pos: math::Vec2 { x: x, y: y },
            bounding_box: math::Rect {
                x: x,
                y: y,
                width: w,
                height: h,
            },
            offset: math::Vec2::zero(),
            clr: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            selected: false,
            derived: AnyComponent::Empty,
        };
    }

    fn with_derived(self, derived: AnyComponent) -> Self {
        let mut c = self;
        c.derived = derived;

        match c.derived {
            AnyComponent::LogicGate {
                chunk: _,
                ref mut input_pins,
                ref mut output_pin,
            } => {
                const PIN_RADIUS: f32 = 10 as f32;
                let mid = c.bounding_box.width / 2.0;
                let pin_yoffset = (mid) / 2.0;
                let pin_xoffset = PIN_RADIUS + 5.0;

                input_pins[0].pos = math::Vec2 {
                    x: c.pos.x - pin_xoffset,
                    y: c.pos.y + pin_yoffset,
                };
                input_pins[1].pos = math::Vec2 {
                    x: c.pos.x - pin_xoffset,
                    y: c.pos.y + mid + pin_yoffset,
                };
                output_pin.pos = math::Vec2 {
                    x: c.pos.x + c.bounding_box.width + pin_xoffset,
                    y: c.pos.y + mid,
                };
            }
            _ => {}
        }
        return c;
    }

    fn with_clr(self, clr: Color) -> Self {
        let mut c = self;
        c.clr = clr;
        return c;
    }

    fn should_interact(&self, m_pos: &math::Vec2) -> bool {
        match &self.derived {
            _ => {
                return self.bounding_box.aabb_check(m_pos);
            }
        }
    }

    fn select(&mut self, m_pos: math::Vec2) {
        self.selected = true;
        self.offset = m_pos.sub(self.pos);
    }

    fn deselect(&mut self) {
        self.selected = false;
    }

    fn update(&mut self, dt: f32, m_pos: math::Vec2, cell_size: i32) {
        self.eval();
        if !self.selected {
            return;
        }

        let mut v = m_pos.sub(self.offset);
        v.x = (((v.x as i32) / cell_size) * cell_size) as f32;
        v.y = (((v.y as i32) / cell_size) * cell_size) as f32;
        v = v.sub(self.pos);
        self.offset_position(v);
        match self.derived {
            AnyComponent::LogicGate {
                chunk: _,
                ref mut input_pins,
                ref mut output_pin,
            } => {
                for pin in input_pins {
                    pin.offset_position(v);
                }
                output_pin.offset_position(v);
            }
            _ => {}
        }
    }

    fn offset_position(&mut self, v: math::Vec2) {
        self.pos = self.pos.add(v);
        self.bounding_box.x = self.pos.x;
        self.bounding_box.y = self.pos.y;
    }

    fn eval(&mut self) {
        match self.derived {
            AnyComponent::LogicGate {
                ref mut chunk,
                ref input_pins,
                ref mut output_pin,
            } => {
                chunk.clear_stack();
                if let (AnyComponent::Pin(in0), AnyComponent::Pin(in1)) =
                    (&input_pins[0].derived, &input_pins[1].derived)
                {
                    chunk.push_stack_value(in0.value);
                    chunk.push_stack_value(in1.value);
                    chunk.reset_ip();
                    if let AnyComponent::Pin(out) = &mut output_pin.derived {
                        out.value = chunk.execute().unwrap();
                    }

                    chunk.reset_ip();
                    chunk.clear_stack();
                }
            }
            _ => {}
        }
    }

    fn draw(&self, theme: &Theme) {
        draw_rectangle(
            self.pos.x,
            self.pos.y,
            self.bounding_box.width,
            self.bounding_box.height,
            self.clr,
        );
        match &self.derived {
            AnyComponent::LogicGate {
                chunk: _,
                input_pins,
                output_pin,
            } => {
                for pin in input_pins {
                    pin.draw(theme);
                }
                output_pin.draw(theme);
            }
            AnyComponent::Pin(pin) => {
                let out_clr = match pin.value {
                    Value::Void => self.clr,
                    Value::Bit(bit) => {
                        if bit.is_true() {
                            Color {
                                r: 1.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }
                        } else {
                            self.clr
                        }
                    }
                };
                draw_circle(self.pos.x, self.pos.y, pin.radius, out_clr);
            }
            _ => {}
        }
    }
}

struct PinComponent {
    kind: PinKind,
    value: Value,
    radius: f32,
}

#[repr(u8)]
enum PinKind {
    In,
    Out,
}

impl PinComponent {
    fn new(kind: PinKind, value: Value) -> Self {
        return PinComponent {
            kind: kind,
            value: value,
            radius: 10.0,
        };
    }
}

#[macroquad::main("HelloWorld")]
async fn main() {
    let mut workbench = Workbench::new();
    let mut chunk = interpreter::Chunk::new();
    chunk.push_op_multiples(&[interpreter::OpCode::And]);

    let component = Component::new(100.0, 100.0, 100.0, 100.0)
        .with_clr(Color {
            r: 0.5,
            g: 0.5,
            b: 0.5,
            a: 1.0,
        })
        .with_derived(AnyComponent::LogicGate {
            chunk: chunk,
            input_pins: [
                Box::new(
                    Component::new(0.0, 0.0, 0.0, 0.0).with_derived(AnyComponent::Pin(
                        PinComponent::new(PinKind::In, Value::Bit(interpreter::BitValue::On)),
                    )),
                ),
                Box::new(
                    Component::new(0.0, 0.0, 0.0, 0.0).with_derived(AnyComponent::Pin(
                        PinComponent::new(PinKind::In, Value::Bit(interpreter::BitValue::On)),
                    )),
                ),
            ],
            output_pin: Box::new(Component::new(0.0, 0.0, 0.0, 0.0).with_derived(
                AnyComponent::Pin(PinComponent::new(
                    PinKind::In,
                    Value::Bit(interpreter::BitValue::On),
                )),
            )),
        });
    workbench.add_component(component);
    loop {
        workbench.handle_input();
        workbench.update(0.0);
        clear_background(workbench.theme.background_clr);

        workbench.draw();
        next_frame().await;
    }
}
