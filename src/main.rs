use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::style::{Color, Stylize};
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::{Canvas, Line, Points};
use ratatui::{DefaultTerminal, Frame};
use std::time::Duration;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}
fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut cube = Mesh3d::cube();
    let mut animation_interrupted = false;
    loop {
        if !animation_interrupted {
            cube.change_rotation_z(0.02);
        }
        terminal.draw(|frame| render(frame, &cube))?;
        if event::poll(Duration::from_millis(8))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('a') => {
                        animation_interrupted = true;
                        cube.change_rotation_z(-0.02);
                    }
                    KeyCode::Char('d') => {
                        animation_interrupted = true;
                        cube.change_rotation_z(0.02);
                    }
                    KeyCode::Char('q') => break Ok(()),
                    _ => {}
                }
            }
        }
    }
}

struct Mesh3d {
    position: (f64, f64, f64),
    rotation: (f64, f64, f64),
    scale: (f64, f64, f64),
    coords: Vec<(f64, f64, f64)>,
    edges: Vec<Vec<usize>>,
}
impl Mesh3d {
    fn cube() -> Self {
        Self {
            position: (0., 2.8, -0.2),
            rotation: (0.6, 0.6, 0.),
            scale: (1., 1., 1.),
            coords: Vec::from([
                (1., -1., 1.),
                (1., -1., -1.),
                (-1., -1., -1.),
                (-1., -1., 1.),
                (1., 1., 1.),
                (1., 1., -1.),
                (-1., 1., -1.),
                (-1., 1., 1.),
            ]),
            edges: vec![
                vec![0, 1, 2, 3],
                vec![4, 5, 6, 7],
                vec![0, 4],
                vec![1, 5],
                vec![2, 6],
                vec![3, 7],
            ],
        }
    }
    fn change_rotation_z(&mut self, amount: f64) {
        self.rotation.2 += amount;
    }
    fn rotate(&self, angles: (f64, f64, f64)) -> Vec<(f64, f64, f64)> {
        self.coords
            .iter()
            .map(|c| {
                (
                    c.0,
                    c.1 * angles.0.cos() - c.2 * angles.0.sin(),
                    c.1 * angles.0.sin() + c.2 * angles.0.cos(),
                )
            })
            .map(|c| {
                (
                    c.0 * angles.1.cos() - c.2 * angles.1.sin(),
                    c.1,
                    c.0 * angles.1.sin() + c.2 * angles.1.cos(),
                )
            })
            .map(|c| {
                (
                    c.0 * angles.2.cos() - c.1 * angles.2.sin(),
                    c.0 * angles.2.sin() + c.1 * angles.2.cos(),
                    c.2,
                )
            })
            .collect()
    }
    fn get_render_data(&self) -> RenderData {
        let coords: Vec<(f64, f64)> = self
            .rotate(self.rotation)
            .iter_mut()
            .map(|c| {
                (
                    c.0 + self.position.0,
                    c.1 + self.position.1,
                    c.2 + self.position.2,
                )
            })
            .map(|c| (c.0 * self.scale.0, c.1 * self.scale.1, c.2 * self.scale.2))
            .map(|c| (c.0 / c.1, c.2 / c.1)) //TODO: this can crash
            .collect();
        let edges: Vec<(f64, f64, f64, f64)> = self
            .edges
            .clone()
            .into_iter()
            .map(|e| {
                if e.len() == 2 {
                    Vec::from([(e[0], e[1])])
                } else {
                    let mut _e = e.clone();
                    _e.push(e[0]);
                    _e.windows(2).map(|w| (w[0], w[1])).collect()
                }
            })
            .flatten()
            .map(|v| (coords[v.0].0, coords[v.0].1, coords[v.1].0, coords[v.1].1))
            .collect();

        RenderData { coords, edges }
    }
}
struct RenderData {
    pub coords: Vec<(f64, f64)>,
    pub edges: Vec<(f64, f64, f64, f64)>,
}

fn render(frame: &mut Frame, cube: &Mesh3d) {
    let render_data = cube.get_render_data();
    let area = frame.area();
    let cell_aspect_correction = 2.0;
    let aspect_ratio = (area.width as f64) / (area.height as f64 * cell_aspect_correction);
    let canvas = Canvas::default()
        .x_bounds([-2.0 * aspect_ratio, 2.0 * aspect_ratio])
        .y_bounds([-2.0, 2.0])
        .marker(Marker::Braille)
        .paint(|ctx| {
            //ctx.layer();
            for e in &render_data.edges {
                ctx.draw(&Line::new(e.0, e.1, e.2, e.3, Color::Blue));
            }
            ctx.draw(&Points {
                coords: &render_data.coords,
                color: Color::Red,
            });
        });
    frame.render_widget(canvas, frame.area());
}
