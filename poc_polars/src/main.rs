// Add the dependencies to your Cargo.toml
// polars = { version = "0.36.2", features = ["lazy", "ndarray"] }
// chrono = "0.4.31"
// ndarray = "0.14"
// ndarray-linalg = "0.16.0"
// plotly = "0.5.1"

use polars::prelude::*;
use plotly::common::{Mode, Title};
use plotly::common::PlotType::Scatter;
use plotly::layout::{Axis, Layout};
use plotly::Plot;

fn main() {
    // Create a Polars DataFrame with x values
    let x = (0..100).map(|i| -10.0 + 0.2 * i as f64).collect::<Vec<_>>();
    let series = Series::new("x", x);

    let df = DataFrame::new(vec![series]).unwrap();

    // Apply the sigmoid function to the x values
    let z = df.column("x").unwrap().clone()
        .apply(|v| 1.0 / (1.0 + f64::exp(-v)));

    // Create a Plotly scatter plot
    let trace = Scatter::new(x.clone(), z.to_vec())
        .mode(Mode::Lines)
        .name("Sigmoid(X)");

    let layout = Layout::new()
        .x_axis(Axis::new().title(Title::from("x")))
        .y_axis(Axis::new().title(Title::from("Sigmoid(X)")));

    let plot = Plot::new().set_layout(layout).add_trace(trace).set_title(Title::new("Sigmoid Function"));

    // Save the plot to an HTML file
    plot.to_html("sigmoid_plot.html").unwrap();
}
