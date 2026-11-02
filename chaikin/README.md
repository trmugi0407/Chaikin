# Chaikin Algorithm

A Rust application that demonstrates **Chaikin's Corner Cutting Algorithm** by creating and animating smooth curves from user-defined control points.

## Features

- Add control points using the left mouse button.
- Display control points as circles.
- Start the animation using the **Space key**.
- Animate the curve through seven iterations.
- Display a straight line when exactly two points are added.
- Handle empty input and a single point safely.
- Move existing control points by clicking and dragging.
- Clear the drawing using the **C key**.
- Exit the application using the **Escape key**.

## Requirements

- Rust
- Cargo
- Macroquad 0.4.16

## Project Structure

```text
chaikin/
├── Cargo.toml
├── Cargo.lock
├── README.md
└── src/
    └── main.rs
```

## Installation

Open the project directory:

```bash
cd chaikin
```

Check Rust and Cargo:

```bash
rustc --version
cargo --version
```

Download and compile the dependencies:

```bash
cargo build
```

## Run the Application

Start the application:

```bash
cargo run
```

## Controls

| Action | Control |
|---|---|
| Add control points | Left mouse click |
| Start animation | Space |
| Move control points | Click and drag |
| Clear the canvas | C |
| Exit application | Escape |

**Note:** Enter is not used to start the animation. Press Space instead.

## Animation Behaviour

The application handles different numbers of control points:

- **0 points:** Displays an empty drawing area. Add points before starting.
- **1 point:** Displays the single control point.
- **2 points:** Displays a straight line.
- **3 or more points:** Runs seven iterations of Chaikin's algorithm.

After the seventh iteration, the animation starts again from the first iteration.

## Chaikin's Algorithm

Chaikin's Corner Cutting Algorithm creates smoother curves by replacing each pair of consecutive points with two new points.

For two consecutive points, \(P_0\) and \(P_1\), the new points are:

\[
Q = 0.75P_0 + 0.25P_1
\]

\[
R = 0.25P_0 + 0.75P_1
\]

The new points are connected to form a smoother curve.

For an open curve with \(N\) points, one iteration produces:

\[
2N - 2
\]

points.

## Testing

Run the unit tests:

```bash
cargo test
```

Check whether the project compiles:

```bash
cargo check
```

Format the Rust code:

```bash
cargo fmt
```

## Dependencies

This project uses:

- **Macroquad:** Provides the application window, graphics, drawing, and mouse and keyboard input.
