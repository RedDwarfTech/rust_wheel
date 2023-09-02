use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct SSEMessage {
    pub event: Option<&'static str>,
    pub data: &'static str,
    pub id: Option<&'static str>,
    pub retry: Option<u32>,
}

impl From<&'static str> for SSEMessage {
    fn from(data: &'static str) -> Self {
        Self {
            event: Some(""),
            data: data,
            id: Some(""),
            retry: None,
        }
    }
}

impl SSEMessage {
    pub fn from_data(data: &'static str, event_type: &'static str) -> Self {
        Self {
            event: Some(event_type),
            data: data,
            id: None,
            retry: Some(5000),
        }
    }
}

impl fmt::Display for SSEMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(event) = &self.event {
            write!(f, "event:{}\n", event)?;
        }
        write!(f, "data:{}\n", self.data)?;
        if let Some(id) = &self.id {
            write!(f, "id:{}\n", id)?;
        }
        if let Some(retry) = &self.retry {
            write!(f, "retry:{}\n", retry)?;
        }
        write!(f, "\n")
    }
}
