
use crate::Value;

impl From<f64> for Value {
    fn from(float: f64) -> Self {
        Self::float(float.into())
    }
}
