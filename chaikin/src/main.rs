
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

// CHAikin's corner-cutting algorithm.
//
// For every pair of adjacent points:
// Q = 75% P0 + 25% P1
// R = 25% P0 + 75% P1
//
// An open curve with N points produces 2N - 2 points.

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

// Generate the original curve and seven iterations.

fn build_steps(points: &[Point]) -> Vec<Vec<Point>> {
    let mut steps = vec![points.to_vec()];
    let mut current = points.to_vec();

    for _ in 0..MAX_STEPS {
        current = chaikin(&current);
        steps.push(current.clone());
    }

    steps
}

// Draw lines connecting the points.

fn draw_curve(points: &[Point], color: Color) {
    for pair in points.windows(2) {
        draw_line(
            pair[0].x,
            pair[0].y,
            pair[1].x,
            pair[1].y,
            2.5,
            color,
        );
    }
}

// Draw circles around the original control points.

fn draw_control_points(points: &[Point]) {
    for (index, point) in points.iter().enumerate() {
        draw_circle_lines(
            point.x,
            point.y,
            POINT_RADIUS + 3.0,
            2.0,
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
            point.y - 8.0,
            18.0,
            WHITE,
        );
    }
}

// Configure the application window.

fn window_conf() -> Conf {
    Conf {
        window_title: "Chaikin Algorithm".to_owned(),
        window_width: 1000,
        window_height: 700,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut control_points: Vec<Point> = Vec::new();

    let mut steps: Vec<Vec<Point>> = vec![Vec::new()];

    let mut animation_running = false;
    let mut current_step: usize = 0;
    let mut elapsed = 0.0_f32;

    let mut dragging: Option<usize> = None;

    let mut status = String::from(
        "Click to add points. Press ENTER to animate.",
    );

    loop {
        let dt = get_frame_time();

        // ESCAPE: quit the application.

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // C: clear the canvas.

        if is_key_pressed(KeyCode::C) {
            control_points.clear();

            steps = vec![Vec::new()];

            animation_running = false;
            current_step = 0;
            elapsed = 0.0;
            dragging = None;

            status = String::from(
                "Canvas cleared. Add new points.",
            );
        }

        // ENTER or SPACE: start the animation.

        if is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::Space)
        {
            if control_points.is_empty() {
                animation_running = false;

                status = String::from(
                    "No points selected. Draw points first.",
                );
            } else if control_points.len() == 1 {
                animation_running = false;
                current_step = 0;

                status = String::from(
                    "One point: nothing to animate.",
                );
            } else if control_points.len() == 2 {
                animation_running = false;
                current_step = 0;

                status = String::from(
                    "Two points: straight line.",
                );
            } else {
                steps = build_steps(&control_points);

                current_step = 1;
                elapsed = 0.0;

                animation_running = true;

                status = String::from(
                    "ANIMATION STARTED!",
                );
            }
        }

        // GET MOUSE POSITION.
        // Keep these variables inside the main loop so
        // both clicking and dragging can use them.

        let (mouse_x, mouse_y) = mouse_position();

        let mouse = Point::new(mouse_x, mouse_y);

        // LEFT CLICK:
        // Select an existing point or add a new point.

        if is_mouse_button_pressed(MouseButton::Left) {
            let mut nearest: Option<(usize, f32)> = None;

            for (index, point) in control_points.iter().enumerate() {
                let distance = point.distance_to(mouse);

                if distance <= DRAG_RADIUS {
                    match nearest {
                        Some((_, best_distance))
                            if best_distance <= distance => {}
                        _ => nearest = Some((index, distance)),
                    }
                }
            }

            if let Some((index, _)) = nearest {
                dragging = Some(index);

                animation_running = false;
                current_step = 0;
                elapsed = 0.0;

                status = String::from(
                    "Dragging control point.",
                );
            } else {
                control_points.push(mouse);

                steps = build_steps(&control_points);

                animation_running = false;
                current_step = 0;
                elapsed = 0.0;

                status = String::from(
                    "Point added. Press ENTER to animate.",
                );
            }
        }

        // DRAG:
        // Move the selected point in real time.

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(index) = dragging {
                control_points[index] = mouse;

                steps = build_steps(&control_points);

                animation_running = false;
                current_step = 0;
                elapsed = 0.0;
            }
        }

        // RELEASE:
        // Stop dragging.

        if is_mouse_button_released(MouseButton::Left) {
            dragging = None;
        }

        // ANIMATION:
        // Display iterations 1, 2, 3, 4, 5, 6, 7.
        // Restart at iteration 1.

        if animation_running {
            elapsed += dt;

            if elapsed >= STEP_DURATION {
                elapsed -= STEP_DURATION;

                if current_step >= MAX_STEPS {
                    current_step = 1;
                } else {
                    current_step += 1;
                }
            }
        }

        // DRAW BACKGROUND.

        clear_background(Color::from_rgba(25, 30, 42, 255));

        // DRAW THE CURVE.

        if control_points.len() == 1 {
            draw_circle(
                control_points[0].x,
                control_points[0].y,
                POINT_RADIUS,
                YELLOW,
            );
        } else if control_points.len() == 2 {
            draw_curve(&control_points, SKYBLUE);
        } else if control_points.len() >= 3 {
            if animation_running {
                // Original polygon.
                draw_curve(&control_points, DARKGRAY);

                // Current Chaikin curve.
                draw_curve(&steps[current_step], YELLOW);
            } else {
                draw_curve(&control_points, GRAY);
            }
        }

        // Always show the original control points.

        draw_control_points(&control_points);

        // DISPLAY INSTRUCTIONS.

        draw_text(
            "LEFT CLICK: add point | DRAG: move point",
            20.0,
            30.0,
            22.0,
            WHITE,
        );

        draw_text(
            "ENTER / SPACE: animate | C: clear | ESC: quit",
            20.0,
            58.0,
            22.0,
            WHITE,
        );

        draw_text(
            &format!(
                "Control points: {}",
                control_points.len()
            ),
            20.0,
            88.0,
            20.0,
            YELLOW,
        );

        if animation_running {
            draw_text(
                &format!(
                    "Chaikin iteration: {}/{}",
                    current_step,
                    MAX_STEPS
                ),
                20.0,
                120.0,
                25.0,
                GREEN,
            );
        }

        draw_text(
            &status,
            20.0,
            155.0,
            18.0,
            WHITE,
        );

        next_frame().await;
    }
}

// UNIT TESTS.

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.0001;

    fn assert_point(actual: Point, expected: Point) {
        assert!(
            (actual.x - expected.x).abs() < EPSILON,
            "Expected x={}, got {}",
            expected.x,
            actual.x
        );

        assert!(
            (actual.y - expected.y).abs() < EPSILON,
            "Expected y={}, got {}",
            expected.y,
            actual.y
        );
    }

    // Test the 25% and 75% coordinates.

    #[test]
    fn test_one_iteration_calculates_correct_points() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(8.0, 4.0),
            Point::new(8.0, 12.0),
        ];

        let result = chaikin(&points);

        assert_eq!(result.len(), 4);

        assert_point(result[0], Point::new(2.0, 1.0));
        assert_point(result[1], Point::new(6.0, 3.0));
        assert_point(result[2], Point::new(8.0, 6.0));
        assert_point(result[3], Point::new(8.0, 10.0));
    }

    // Test the number of points after each iteration.

    #[test]
    fn test_point_count_grows_correctly() {
        let mut points = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 10.0),
        ];

        let expected_counts = [4, 6, 10, 18];

        for expected in expected_counts {
            points = chaikin(&points);

            assert_eq!(points.len(), expected);
        }
    }

    // Empty input must not crash.

    #[test]
    fn test_empty_input() {
        let points: Vec<Point> = Vec::new();

        assert!(chaikin(&points).is_empty());
    }

    // One point must remain unchanged.

    #[test]
    fn test_single_point_is_unchanged() {
        let points = vec![
            Point::new(5.0, 9.0),
        ];

        assert_eq!(chaikin(&points), points);
    }

    // Two points must produce two points on a straight line.

    #[test]
    fn test_two_points_produce_straight_line() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(100.0, 0.0),
        ];

        let result = chaikin(&points);

        assert_eq!(result.len(), 2);

        assert_point(result[0], Point::new(25.0, 0.0));
        assert_point(result[1], Point::new(75.0, 0.0));
    }

    // Test all seven iterations.

    #[test]
    fn test_seven_iterations_are_generated() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(100.0, 100.0),
            Point::new(200.0, 0.0),
        ];

        let steps = build_steps(&points);

        assert_eq!(steps.len(), 8);

        assert_eq!(steps[0].len(), 3);
        assert_eq!(steps[1].len(), 4);
        assert_eq!(steps[7].len(), 130);
    }
}
