use hdk3::prelude::*;

use crate::{PrivateShortFormExpression, ShortFormExpression, ShortFormExpressionData};

use crate::utils::err;

#[derive(SerializedBytes, Serialize, Deserialize, Clone)]
pub struct CreateExpression {
    pub data: String,
    pub author: Agent,
    pub timestamp: String,
    pub proof: ExpressionProof,
}

#[derive(SerializedBytes, Serialize, Deserialize, Clone)]
pub struct ExpressionProof {
    pub signature: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct Agent {
    pub did: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

impl TryFrom<CreateExpression> for ShortFormExpression {
    type Error = HdkError;

    fn try_from(content: CreateExpression) -> Result<Self, Self::Error> {
        let expression: ShortFormExpressionData = serde_json::from_str(&content.data)
            .map_err(|_| err("Could not serialized content into ShortForm expression type"))?;

        Ok(ShortFormExpression {
            data: expression,
            author: content.author,
            timestamp: content.timestamp,
            proof: content.proof,
        })
    }
}

impl From<ShortFormExpression> for PrivateShortFormExpression {
    fn from(content: ShortFormExpression) -> Self {
        PrivateShortFormExpression {
            data: content.data,
            author: content.author,
            timestamp: content.timestamp,
            proof: content.proof,
        }
    }
}

#[derive(SerializedBytes, Serialize, Deserialize)]
pub struct CreatePrivateExpression {
    pub data: String,
    pub author: Agent,
    pub timestamp: String,
    pub proof: ExpressionProof,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct GetByAuthor {
    pub author: String,
    pub page_size: usize,
    pub page_number: usize,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct SendPrivate {
    pub to: AgentPubKey,
    pub expression: CreateExpression,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct Inbox {
    pub from: Option<String>,
    pub page_size: usize,
    pub page_number: usize,
}
