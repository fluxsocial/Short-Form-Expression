use hc_utils::WrappedDnaHash;
use hdk3::prelude::*;
use holo_hash::DnaHash;

use crate::{PrivateShortFormExpression, ShortFormExpression};

#[derive(SerializedBytes, Serialize, Deserialize)]
pub struct ExpressionResponse {
    //#[serde(flatten)]
    pub expression_data: ShortFormExpression,
    pub holochain_data: HolochainData,
}

#[derive(SerializedBytes, Serialize, Deserialize)]
pub struct PrivateExpressionResponse {
    //#[serde(flatten)]
    pub expression_data: PrivateShortFormExpression,
    pub holochain_data: HolochainData,
}

#[derive(SerializedBytes, Serialize, Deserialize)]
pub struct HolochainData {
    pub element: Element,
    pub expression_dna: DnaHash,
    pub creator: AgentPubKey,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// #[derive(SerializedBytes, Serialize, Deserialize)]
// pub struct WrappedExpressionResponse(pub ExpressionResponse);

#[derive(SerializedBytes, Serialize, Deserialize)]
pub struct ManyExpressionResponse(pub Vec<ExpressionResponse>);

#[derive(SerializedBytes, Serialize, Deserialize)]
pub struct ManyPrivateExpressionResponse(pub Vec<PrivateExpressionResponse>);

#[derive(SerializedBytes, Serialize, Deserialize)]
pub struct MaybeExpression(pub Option<ExpressionResponse>);

// #[derive(SerializedBytes, Serialize, Deserialize)]
// pub struct WrappedPrivateExpressionResponse(pub PrivateShortFormExpression);

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct ManyDhtHash(pub Vec<WrappedDnaHash>);
