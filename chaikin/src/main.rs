
use macroquad::prelude::*;

const MAX_STEPS: usize = 7;
const STEP_DURATION: f32 = 1.0;
const POINT_RADIUS: f32 = 5.0;
const DRAG_RADIUS: f32 = 14.0;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn distance_to(self, other: Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        (dx * dx + dy * dy).sqrt()
    }
}

// Chaikin's Corner Cutting Algorithm.
fn chaikin(points: &[Point]) -> Vec<Point> {
    if points.len() < 2 {
        return points.to_vec();
    }

    let mut result = Vec::with_capacity(2 * (points.len() - 1));

    for pair in points.windows(2) {
        let p0 = pair[0];
        let p1 = pair[1];

        let q = Point::new(
            0.75 * p0.x + 0.25 * p1.x,
            0.75 * p0.y + 0.25 * p1.y,
        );

        let r = Point::new(
            0.25 * p0.x + 0.75 * p1.x,
            0.25 * p0.y + 0.75 * p1.y,
        );

        result.push(q);
        result.push(r);
    }

    result
}

// Build the original curve and seven smoothed versions.
fn build_steps(points: &[Point]) -> Vec<Vec<Point>> {
    let mut steps = Vec::new();

    steps.push(points.to_vec());

    let mut current = points.to_vec();

    for _ in 0..MAX_STEPS {
        current = chaikin(&current);
        steps.push(current.clone());
    }

    steps
}

// Draw lines connecting the points.
fn draw_curve(points: &[Point], color: Color, thickness: f32) {
    for pair in points.windows(2) {
        draw_line(
            pair[0].x,
            pair[0].y,
            pair[1].x,
            pair[1].y,
            thickness,
            color,
        );
    }
}

// Draw the control points as circles.
fn draw_control_points(points: &[Point]) {
    for (index, point) in points.iter().enumerate() {
        draw_circle(
            point.x,
            point.y,
            POINT_RADIUS + 3.0,
            WHITE,
        );

        draw_circle(
            point.x,
            point.y,
            POINT_RADIUS,
            YELLOW,
        );

        draw_text(
            &format!("{}", index + 1),
            point.x + 10.0,
            point.y - 10.0,
            18.0,
            WHITE,
        );
    }
}

// Window settings.
fn window_conf() -> Conf {
    Conf {
        window_title: "Chaikin Algorithm".to_string(),
        window_width: 1000,
        window_height: 700,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut control_points: Vec<Point> = Vec::new();

    let mut steps: Vec<Vec<Point>> = Vec::new();

    let mut current_step: usize = 0;

    let mut elapsed: f32 = 0.0;

    let mut animation_running = false;

    let mut dragging: Option<usize> = None;

    let mut status = String::from(
        "Click to add points. Press SPACE to animate."
    );

    loop {
        // Exit the application.
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Clear the canvas.
        if is_key_pressed(KeyCode::C) {
            control_points.clear();
            steps.clear();

            current_step = 0;
            elapsed = 0.0;
            animation_running = false;
            dragging = None;

            status = String::from("Canvas cleared. Add new points.");
        }

        // SPACE is the only animation-start key.
        if is_key_pressed(KeyCode::Space) {
            if control_points.is_empty() {
                animation_running = false;
                status = String::from(
                    "Please add points before starting."
                );
            } else if control_points.len() == 1 {
                animation_running = false;
                current_step = 0;
                status = String::from(
                    "One point selected. Nothing to animate."
                );
            } else if control_points.len() == 2 {
                animation_running = false;
                current_step = 0;
                status = String::from(
                    "Two points: straight line."
                );
            } else {
                steps = build_steps(&control_points);

                current_step = 1;
                elapsed = 0.0;
                animation_running = true;

                status = String::from(
                    "SPACE PRESSED! ANIMATION STARTED!"
                );
            }
        }

        // Get the current mouse position.
        let (mouse_x, mouse_y) = mouse_position();
        let mouse = Point::new(mouse_x, mouse_y);

        // Add a point or select an existing point.
        if is_mouse_button_pressed(MouseButton::Left) {
            let mut selected_point: Option<usize> = None;
            let mut shortest_distance = DRAG_RADIUS;

            for (index, point) in control_points.iter().enumerate() {
                let distance = point.distance_to(mouse);

                if distance < shortest_distance {
                    shortest_distance = distance;
                    selected_point = Some(index);
                }
            }

            if let Some(index) = selected_point {
                dragging = Some(index);
                animation_running = false;

                status = String::from("Moving control point.");
            } else {
                control_points.push(mouse);

                animation_running = false;
                current_step = 0;

                status = format!(
                    "{} control point(s) added.",
                    control_points.len()
                );
            }
        }

        // Move a selected point while the mouse is held down.
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(index) = dragging {
                if index < control_points.len() {
                    control_points[index] = mouse;
                    animation_running = false;
                    current_step = 0;
                }
            }
        }

        // Stop dragging when the mouse button is released.
        if is_mouse_button_released(MouseButton::Left) {
            dragging = None;
        }

        // Update the animation.
        if animation_running {
            elapsed += get_frame_time();

            while elapsed >= STEP_DURATION {
                elapsed -= STEP_DURATION;
                current_step += 1;

                if current_step > MAX_STEPS {
                    current_step = 1;
                }
            }
        }

        // Draw the application.
        clear_background(Color::from_rgba(25, 28, 38, 255));

        // Draw the curve.
        if control_points.len() == 1 {
            let point = control_points[0];

            draw_circle(
                point.x,
                point.y,
                POINT_RADIUS,
                YELLOW,
            );
        } else if control_points.len() == 2 {
            draw_curve(&control_points, YELLOW, 3.0);
        } else if control_points.len() >= 3 {
            // Show the original control polygon.
            draw_curve(
                &control_points,
                DARKGRAY,
                2.0,
            );

            // Show the current smoothed curve.
            if animation_running && !steps.is_empty() {
                if current_step < steps.len() {
                    draw_curve(
                        &steps[current_step],
                        YELLOW,
                        3.0,
                    );
                }
            } else {
                draw_curve(
                    &control_points,
                    YELLOW,
                    3.0,
                );
            }
        }

        // Always display the original control points.
        draw_control_points(&control_points);

        // Display instructions.
        draw_text(
            "CHAIKIN ALGORITHM",
            20.0,
            30.0,
            28.0,
            WHITE,
        );

        draw_text(
            "Left click: Add points | Drag: Move points",
            20.0,
            60.0,
            18.0,
            LIGHTGRAY,
        );

        draw_text(
            "SPACE: Animate | C: Clear | ESC: Exit",
            20.0,
            85.0,
            18.0,
            LIGHTGRAY,
        );

        draw_text(
            &status,
            20.0,
            screen_height() - 45.0,
            20.0,
            GREEN,
        );

        if animation_running {
            draw_text(
                &format!(
                    "Iteration: {} / {}",
                    current_step,
                    MAX_STEPS
                ),
                20.0,
                120.0,
                20.0,
                YELLOW,
            );
        }

        next_frame().await;
    }
}

// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chaikin_coordinates() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(8.0, 4.0),
            Point::new(8.0, 12.0),
        ];

        let result = chaikin(&points);

        assert_eq!(result.len(), 4);

        assert_eq!(result[0], Point::new(2.0, 1.0));
        assert_eq!(result[1], Point::new(6.0, 3.0));
        assert_eq!(result[2], Point::new(8.0, 6.0));
        assert_eq!(result[3], Point::new(8.0, 10.0));
    }

    #[test]
    fn test_point_count_increases() {
        let mut points = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 10.0),
            Point::new(20.0, 0.0),
        ];

        let expected_counts = vec![4, 6, 10, 18];

        for expected in expected_counts {
            points = chaikin(&points);
            assert_eq!(points.len(), expected);
        }
    }

    #[test]
    fn test_empty_points() {
        let points: Vec<Point> = Vec::new();

        let result = chaikin(&points);

        assert!(result.is_empty());
    }

    #[test]
    fn test_one_point() {
        let points = vec![Point::new(5.0, 10.0)];

        let result = chaikin(&points);

        assert_eq!(result, points);
    }

    #[test]
    fn test_two_points() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(100.0, 0.0),
        ];

        let result = chaikin(&points);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Point::new(25.0, 0.0));
        assert_eq!(result[1], Point::new(75.0, 0.0));
    }

    #[test]
    fn test_seven_iterations() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 20.0),
            Point::new(20.0, 0.0),
        ];

        let steps = build_steps(&points);

        assert_eq!(steps.len(), 8);
        assert_eq!(steps[0].len(), 3);
        assert_eq!(steps[1].len(), 4);
        assert_eq!(steps[7].len(), 130);
    }
}
