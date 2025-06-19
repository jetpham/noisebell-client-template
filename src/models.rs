use serde::{Deserialize, Serialize, Deserializer, Serializer};
use serde::de::{self, Visitor};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum WebhookEvent {
    Open,
    Closed,
    Unknown(String),
}

impl fmt::Display for WebhookEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebhookEvent::Open => write!(f, "open"),
            WebhookEvent::Closed => write!(f, "closed"),
            WebhookEvent::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl WebhookEvent {
    pub fn as_str(&self) -> &str {
        match self {
            WebhookEvent::Open => "open",
            WebhookEvent::Closed => "closed",
            WebhookEvent::Unknown(s) => s.as_str(),
        }
    }
}

impl Serialize for WebhookEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            WebhookEvent::Open => serializer.serialize_str("open"),
            WebhookEvent::Closed => serializer.serialize_str("closed"),
            WebhookEvent::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WebhookEventVisitor;

        impl<'de> Visitor<'de> for WebhookEventVisitor {
            type Value = WebhookEvent;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a webhook event type")
            }

            fn visit_str<E>(self, value: &str) -> Result<WebhookEvent, E>
            where
                E: de::Error,
            {
                match value {
                    "open" => Ok(WebhookEvent::Open),
                    "closed" => Ok(WebhookEvent::Closed),
                    other => Ok(WebhookEvent::Unknown(other.to_string())),
                }
            }
        }

        deserializer.deserialize_str(WebhookEventVisitor)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookPayload {
    pub event: WebhookEvent,
    pub timestamp: String,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub status: String,
    pub data: StatusData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusData {
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookListResponse {
    pub status: String,
    pub data: WebhookListData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookListData {
    pub webhooks: Vec<WebhookInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookInfo {
    pub url: String,
    pub created_at: String,
} 