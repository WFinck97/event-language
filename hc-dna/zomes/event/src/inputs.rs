use chrono::{DateTime, Utc};
use hdk::prelude::*;

use crate::{PrivateEventExpression, EventExpression, EventExpressionData};

use crate::utils::err;

#[derive(SerializedBytes, Serialize, Deserialize, Clone, Debug)]
pub struct CreateExpression {
    pub data: String,
    pub author: String,
    pub timestamp: DateTime<Utc>,
    pub proof: ExpressionProof,
}

#[derive(SerializedBytes, Serialize, Deserialize, Clone, Debug)]
pub struct ExpressionProof {
    pub signature: String,
    pub key: String,
}

impl TryFrom<CreateExpression> for EventExpression {
    type Error = WasmError;

    fn try_from(content: CreateExpression) -> Result<Self, Self::Error> {
        let expression: EventExpressionData = serde_json::from_str(&content.data)
            .map_err(|_| err("Could not serialized content into Event expression type"))?;

        Ok(EventExpression {
            data: expression,
            author: content.author,
            timestamp: content.timestamp,
            proof: content.proof,
        })
    }
}

impl From<EventExpression> for PrivateEventExpression {
    fn from(content: EventExpression) -> Self {
        PrivateEventExpression {
            data: content.data,
            author: content.author,
            timestamp: content.timestamp,
            proof: content.proof,
        }
    }
}

#[derive(SerializedBytes, Serialize, Deserialize, Debug)]
pub struct CreatePrivateExpression {
    pub data: String,
    pub author: String,
    pub timestamp: String,
    pub proof: ExpressionProof,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct GetByAuthor {
    pub author: String,
    pub from: DateTime<Utc>,
    pub until: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct SendPrivate {
    pub to: AgentPubKey,
    pub expression: CreateExpression,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct Inbox {
    pub from: Option<String>,
    pub page_size: usize,
    pub page_number: usize,
}
