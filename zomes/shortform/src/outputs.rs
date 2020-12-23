use hc_utils::WrappedDnaHash;
use hdk3::prelude::*;
use meta_traits::Expression;

#[derive(SerializedBytes, Serialize, Deserialize)]
pub struct ExpressionResponse(pub Expression);

#[derive(SerializedBytes, Serialize, Deserialize)]
pub struct ManyExpressionResponse(pub Vec<Expression>);

#[derive(SerializedBytes, Serialize, Deserialize)]
pub struct MaybeExpression(pub Option<Expression>);

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct StringResponse(pub String);

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct ManyDhtHash(pub Vec<WrappedDnaHash>);
