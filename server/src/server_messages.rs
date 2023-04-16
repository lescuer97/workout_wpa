use serde::{Deserialize, Serialize};

const SUCCESS: &str = "success";
const FAILURE: &str = "sailure";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResponseBodyMessage<T> {
    pub status: String,
    pub data: T,
}
impl<T> ResponseBodyMessage<T> {
    pub fn success_message(data: T) -> Self {
        Self {
            status: SUCCESS.to_string(),
            data,
        }
    }
    pub fn fail_message(data: T) -> Self {
        Self {
            status: FAILURE.to_string(),
            data,
        }
    }
}
