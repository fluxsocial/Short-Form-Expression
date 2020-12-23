use hdk3::prelude::*;
use meta_traits::Identity;

#[derive(SerializedBytes, Serialize, Deserialize)]
pub struct CreateExpression {
    pub content: String,
}

#[derive(SerializedBytes, Serialize, Deserialize)]
pub struct CreatePrivateExpression {
    pub content: String,
    pub sender: Identity,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct GetByAuthor {
    pub author: Identity,
    pub page_size: usize,
    pub page_number: usize,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct SendPrivate {
    pub to: Identity,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct Inbox {
    pub from: Option<Identity>,
    pub page_size: usize,
    pub page_number: usize,
}
