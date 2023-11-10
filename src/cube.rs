use crate::point3d::Point3D;
use crossterm::style::Color;
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute, style,
    terminal::{self, ClearType},
};
use rand::Rng;
use std::io::Write;

static TITLE: &str = "Press ESC to exit | Rotating Cube Demo by Jeremy Kescher and Markus Mayer";

struct CornerData {
    a: Point3D,
    b: Point3D,
}

pub struct Cube {
    lines: Vec<CornerData>,
    key_press: KeyEvent,
    alt_down: bool,
    shift_down: bool,
    ang_x: f32,
    ang_y: f32,
    ang_z: f32,
    pub exit: bool,
    manual_control: bool,
    rotation_factor: f32,
}

impl Cube {
    pub fn new() -> Cube {
        let mut corners = Vec::new();

        // List of all possible corners of a cube in 3D space
        let cube_corners = vec![
            Point3D::new(-1.0, -1.0, -1.0),
            Point3D::new(1.0, -1.0, -1.0),
            Point3D::new(1.0, -1.0, 1.0),
            Point3D::new(-1.0, -1.0, 1.0),
            Point3D::new(-1.0, 1.0, 1.0),
            Point3D::new(-1.0, 1.0, -1.0),
            Point3D::new(1.0, 1.0, -1.0),
            Point3D::new(1.0, 1.0, 1.0),
        ];

        // LINQ query to get all corners necessary for 2D space
        for a in &cube_corners {
            for b in &cube_corners {
                if (*a - *b).length() == 2.0 && a.x + a.y + a.z > b.x + b.y + b.z {
                    corners.push(CornerData { a: *a, b: *b });
                }
            }
        }

        Cube {
            lines: corners,
            key_press: KeyEvent::new(KeyCode::Null, KeyModifiers::NONE),
            alt_down: false,
            shift_down: false,
            ang_x: 0.0,
            ang_y: 0.0,
            ang_z: 0.0,
            exit: false,
            manual_control: false,
            rotation_factor: 1.0,
        }
    }

    pub fn init(&self) {
        terminal::enable_raw_mode().unwrap();
        execute!(
            std::io::stdout(),
            terminal::SetTitle(TITLE),
            terminal::SetSize(60, 30),
            terminal::Clear(ClearType::All),
            cursor::Hide,
        )
        .unwrap();
    }

    pub fn set_fullscreen(&self) {
        execute!(std::io::stdout(), terminal::SetSize(60, 30)).unwrap();
    }

    pub fn print_2d_projection(&self) {
        for line in &self.lines {
            for i in 0..25 {
                // Find a point between A and B by following formula p=a+z(b-a) where z
                // is a value between 0 and 1.
                let point = line.a + (i as f32 / 24.0) * (line.b - line.a);
                // Rotates the point relative to all the angles.
                let mut r = point;
                r.rotate_x(self.ang_x);
                r.rotate_y(self.ang_y);
                r.rotate_z(self.ang_z);
                // Projects the point into 2d space. Acts as a kind of camera setting.
                let q = r.project(40.0, 40.0, 350.0, 4.0);
                // Setting the cursor to a projecting position
                let x = (q.x + 400.0) as i16 / 10;
                let y = (q.y + 200.0) as i16 / 10;
                execute!(
                    std::io::stdout(),
                    cursor::MoveTo(x as _, y as _),
                    style::SetForegroundColor(Color::Red),
                    style::SetBackgroundColor(Color::Rgb {
                        r: 20,
                        g: 20,
                        b: 42
                    }),
                    style::Print("•") // █
                )
                .unwrap();
            }
        }

        std::io::stdout().flush().unwrap();
    }

    pub fn handle_input(&mut self) {
        let has_event = event::poll(std::time::Duration::from_millis(25)).unwrap();

        if self.manual_control && has_event {
            if let event::Event::Key(key) = event::read().unwrap() {
                self.key_press = key;
                self.alt_down = self.key_press.modifiers.contains(KeyModifiers::ALT);
                self.shift_down = self.key_press.modifiers.contains(KeyModifiers::SHIFT);

                if self.shift_down {
                    self.rotation_factor = if self.alt_down { 1.0 } else { 0.5 };
                } else if self.alt_down {
                    self.rotation_factor = 2.0;
                } else {
                    self.rotation_factor = 1.0;
                }

                match self.key_press.code {
                    KeyCode::Char('w') => self.ang_x += self.rotation_factor,
                    KeyCode::Char('a') => self.ang_y += self.rotation_factor,
                    KeyCode::Char('s') => self.ang_x -= self.rotation_factor,
                    KeyCode::Char('d') => self.ang_y -= self.rotation_factor,
                    KeyCode::Char('j') => self.ang_z += self.rotation_factor,
                    KeyCode::Char('k') => self.ang_z -= self.rotation_factor,
                    KeyCode::Char('r') => {
                        self.ang_x = 0.0;
                        self.ang_y = 0.0;
                        self.ang_z = 0.0;
                    }
                    KeyCode::Char('m') => self.manual_control = false,
                    KeyCode::Esc => self.exit = true,
                    _ => {}
                }
            }
        } else if !self.manual_control {
            if has_event {
                if let event::Event::Key(key) = event::read().unwrap() {
                    self.key_press = key;
                    match self.key_press.code {
                        KeyCode::Char('m') => self.manual_control = true,
                        KeyCode::Esc => self.exit = true,
                        _ => {}
                    }
                }
            }

            self.ang_x += rand::thread_rng().gen_range(0..=2) as f32;
            self.ang_y += rand::thread_rng().gen_range(0..=2) as f32;
            self.ang_z += rand::thread_rng().gen_range(0..=2) as f32;
        }
    }

    pub fn clear_screen(&self) {
        execute!(std::io::stdout(), terminal::Clear(ClearType::All)).unwrap()
    }

    pub fn exit(&self) {
        execute!(std::io::stdout(), style::ResetColor, cursor::Show).unwrap();
        self.clear_screen();
        terminal::disable_raw_mode().unwrap();
    }
}

impl Drop for Cube {
    fn drop(&mut self) {
        self.exit()
    }
}
