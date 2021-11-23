use hdk::prelude::*;

use crate::{PrivateEventExpression, EventExpression};

#[derive(SerializedBytes, Serialize, Deserialize, Debug)]
pub struct ExpressionResponse {
    //#[serde(flatten)]
    pub expression_data: EventExpression,
    pub holochain_data: HolochainData,
}

#[derive(SerializedBytes, Serialize, Deserialize, Debug)]
pub struct PrivateExpressionResponse {
    //#[serde(flatten)]
    pub expression_data: PrivateEventExpression,
    pub holochain_data: HolochainData,
}

#[derive(SerializedBytes, Serialize, Deserialize, Debug)]
pub struct HolochainData {
    pub element: Element,
    pub expression_dna: String,
    pub creator: AgentPubKey,
    pub created_at: chrono::DateTime<chrono::Utc>,
}


#[derive(SerializedBytes, Serialize, Deserialize, Debug)]
pub struct ManyExpressionResponse(pub Vec<ExpressionResponse>);

#[derive(SerializedBytes, Serialize, Deserialize, Debug)]
pub struct ManyPrivateExpressionResponse(pub Vec<PrivateExpressionResponse>);

#[derive(SerializedBytes, Serialize, Deserialize, Debug)]
pub struct MaybeExpression(pub Option<ExpressionResponse>);

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct ManyDhtHash(pub Vec<String>);
