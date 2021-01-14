use hc_utils::WrappedAgentPubKey;
use hdk3::prelude::*;

mod inputs;
mod methods;
mod outputs;
mod utils;

use inputs::*;
use outputs::*;

/// Expression data this DNA is "hosting"
#[hdk_entry(id = "shortform_expression", visibility = "public")]
pub struct ShortFormExpression {
    data: ShortFormExpressionData,
    author: Agent,
    timestamp: String,
    proof: ExpressionProof,
}

/// Expression data this DNA is "hosting". This variant is private and will be used for p2p messaging.
#[hdk_entry(id = "private_shortform_expression", visibility = "private")]
pub struct PrivateShortFormExpression {
    data: ShortFormExpressionData,
    author: Agent,
    timestamp: String,
    proof: ExpressionProof,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct ShortFormExpressionData {
    background: Vec<String>,
    body: String,
}

#[hdk_entry(id = "acai_agent", visibility = "public")]
pub struct AcaiAgent(pub String);

#[hdk_entry(id = "private_acai_agent", visibility = "private")]
pub struct PrivateAcaiAgent(pub String);

pub struct ExpressionDNA();

entry_defs![
    ShortFormExpression::entry_def(),
    PrivateShortFormExpression::entry_def(),
    AcaiAgent::entry_def(),
    PrivateAcaiAgent::entry_def(),
    Path::entry_def()
];

// Zome functions

/// Run function where zome is init'd by agent. This adds open cap grant for recv_private_expression function
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    // grant unrestricted access to accept_cap_claim so other agents can send us claims
    let mut functions: GrantedFunctions = HashSet::new();
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
    ExpressionDNA::recv_private_expression(create_data)
}

/// Get agent information
#[hdk_extern]
pub fn who_am_i(_: ()) -> ExternResult<WrappedAgentPubKey> {
    let agent_info = agent_info()?;

    Ok(WrappedAgentPubKey(agent_info.agent_initial_pubkey))
}

/// Create an expression and link it to yourself publicly
#[hdk_extern]
pub fn create_public_expression(create_data: CreateExpression) -> ExternResult<ExpressionResponse> {
    Ok(ExpressionDNA::create_public_expression(create_data)?)
}

/// Get expressions authored by a given Agent/Identity
#[hdk_extern]
pub fn get_by_author(get_data: GetByAuthor) -> ExternResult<ManyExpressionResponse> {
    Ok(ManyExpressionResponse(ExpressionDNA::get_by_author(
        get_data.author,
        get_data.page_size,
        get_data.page_number,
    )?))
}

#[hdk_extern]
pub fn get_expression_by_address(address: AnyDhtHash) -> ExternResult<MaybeExpression> {
    Ok(MaybeExpression(ExpressionDNA::get_expression_by_address(
        address,
    )?))
}

/// Send an expression to someone privately p2p
#[hdk_extern]
pub fn send_private(send_data: SendPrivate) -> ExternResult<PrivateShortFormExpression> {
    Ok(ExpressionDNA::send_private(
        send_data.to,
        send_data.expression,
    )?)
}

/// Get private expressions sent to you optionally filtered by sender address
#[hdk_extern]
pub fn inbox(data: Inbox) -> ExternResult<ManyPrivateExpressionResponse> {
    Ok(ManyPrivateExpressionResponse(ExpressionDNA::inbox(
        data.from,
        data.page_size,
        data.page_number,
    )?))
}

/// Allows for describing what other DNA's should be installed in addition to this one
/// This is useful if the implementing expression DNA wishes to create some dependency on another DNA
/// Example could be a storage DNA
#[hdk_extern]
pub fn required_dnas(_: ()) -> ExternResult<ManyDhtHash> {
    Ok(ManyDhtHash(vec![]))
}

// Validation functions
