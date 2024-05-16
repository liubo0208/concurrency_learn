mod metrics;

mod matrix;

mod vector;

pub use matrix::{multiply, Matrix};
pub use metrics::{AmapMetrics, CmapMetrics};
pub use vector::{dot_product, Vector};