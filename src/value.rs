use crate::{error, tensor::Tensor};

pub struct Value<T> {
    pub value: T,
}

impl<T: Tensor> Value<T> {
    pub fn new(value: T) -> Self {
        Value { value }
    }

    pub fn add(&self, other: &Self) -> Self {
        Value::new(self.value.add(&other.value))
    }

    pub fn sub(&self, other: &Self) -> Self {
        Value::new(self.value.sub(&other.value))
    }

    pub fn mul(&self, other: &Self) -> Self {
        Value::new(self.value.mul(&other.value))
    }

    pub fn div(&self, other: &Self) -> error::Result<Self> {
      if other.value.is_zero() {
          return Err(error::Error::DivisionByZeroError);
      }
       
       Ok(Value::new(self.value.div(&other.value)))
    }
}