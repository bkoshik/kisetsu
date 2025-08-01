use serde::Serialize;

pub mod from_str;

#[derive(Serialize, Clone, Debug)]
pub enum Units {
    Metric,
    Imperial,
}