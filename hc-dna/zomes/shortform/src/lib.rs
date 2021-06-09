use chrono::{DateTime, Utc};
use hdk::prelude::*;

mod inputs;
mod methods;
mod outputs;
mod utils;
mod impls;
mod errors;

use inputs::*;
use outputs::*;

/// Expression data this DNA is "hosting"
#[hdk_entry(id = "shortform_expression", visibility = "public")]
#[derive(Clone)]
pub struct ShortFormExpression {
    data: ShortFormExpressionData,
    author: Agent,
    timestamp: DateTime<Utc>,
    proof: ExpressionProof,
}

/// Expression data this DNA is "hosting". This variant is private and will be used for p2p messaging.
#[hdk_entry(id = "private_shortform_expression", visibility = "private")]
pub struct PrivateShortFormExpression {
    data: ShortFormExpressionData,
    author: Agent,
    timestamp: DateTime<Utc>,
    proof: ExpressionProof,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct ShortFormExpressionData {
    background: Vec<String>,
    body: String,
}

#[hdk_entry(id = "private_acai_agent", visibility = "private")]
pub struct PrivateAcaiAgent(pub String);

pub struct ExpressionDNA();

entry_defs![
    ShortFormExpression::entry_def(),
    PrivateShortFormExpression::entry_def(),
    PrivateAcaiAgent::entry_def(),
    Path::entry_def()
];

// Zome functions

/// Run function where zome is init'd by agent. This adds open cap grant for recv_private_expression function
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    // grant unrestricted access to accept_cap_claim so other agents can send us claims
    let mut functions: GrantedFunctions = BTreeSet::new();
    functions.insert((zome_info()?.zome_name, "recv_private_expression".into()));
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn recv_private_expression(create_data: PrivateShortFormExpression) -> ExternResult<()> {
    ExpressionDNA::recv_private_expression(create_data).map_err(|err| WasmError::Host(err.to_string()))
}

/// Create an expression and link it to yourself publicly
#[hdk_extern]
pub fn create_public_expression(create_data: CreateExpression) -> ExternResult<ExpressionResponse> {
    Ok(ExpressionDNA::create_public_expression(create_data).map_err(|err| WasmError::Host(err.to_string()))?)
}

/// Get expressions authored by a given Agent/Identity
#[hdk_extern]
pub fn get_by_author(get_data: GetByAuthor) -> ExternResult<ManyExpressionResponse> {
    Ok(ManyExpressionResponse(ExpressionDNA::get_by_author(
        get_data.author,
        get_data.from,
        get_data.until,
    ).map_err(|err| WasmError::Host(err.to_string()))?))
}

#[hdk_extern]
pub fn get_expression_by_address(address: AnyDhtHash) -> ExternResult<MaybeExpression> {
    Ok(MaybeExpression(ExpressionDNA::get_expression_by_address(
        address,
    ).map_err(|err| WasmError::Host(err.to_string()))?))
}

/// Send an expression to someone privately p2p
#[hdk_extern]
pub fn send_private(send_data: SendPrivate) -> ExternResult<PrivateShortFormExpression> {
    Ok(ExpressionDNA::send_private(
        send_data.to,
        send_data.expression,
    ).map_err(|err| WasmError::Host(err.to_string()))?)
}

/// Get private expressions sent to you optionally filtered by sender address
#[hdk_extern]
pub fn inbox(data: Inbox) -> ExternResult<ManyPrivateExpressionResponse> {
    Ok(ManyPrivateExpressionResponse(ExpressionDNA::inbox(
        data.from,
        data.page_size,
        data.page_number,
    ).map_err(|err| WasmError::Host(err.to_string()))?))
}

/// Allows for describing what other DNA's should be installed in addition to this one
/// This is useful if the implementing expression DNA wishes to create some dependency on another DNA
/// Example could be a storage DNA
#[hdk_extern]
pub fn required_dnas(_: ()) -> ExternResult<ManyDhtHash> {
    Ok(ManyDhtHash(vec![]))
}

// Validation functions
