use hc_utils::WrappedAgentPubKey;
use hdk3::prelude::*;
use meta_traits::ExpressionDao;

mod inputs;
mod methods;
mod outputs;
mod utils;

use inputs::*;
use outputs::*;

/// Expression data this DNA is "hosting"
#[hdk_entry(id = "shortform_expression", visibility = "public")]
pub struct ShortFormExpression {
    background: Vec<String>,
    body: String,
}

/// Expression data this DNA is "hosting". This variant is private and will be used for p2p messaging.
#[hdk_entry(id = "private_shortform_expression", visibility = "private")]
pub struct PrivateShortFormExpression {
    background: Vec<String>,
    body: String,
}

#[hdk_entry(id = "private_agent", visibility = "private")]
pub struct PrivateAgent(AgentPubKey);

pub struct ExpressionDNA();

entry_defs![
    ShortFormExpression::entry_def(),
    PrivateShortFormExpression::entry_def(),
    PrivateAgent::entry_def(),
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
pub fn recv_private_expression(create_data: CreatePrivateExpression) -> ExternResult<()> {
    methods::recv_private_expression(create_data)
}

/// Get agent information
#[hdk_extern]
pub fn who_am_i(_: ()) -> ExternResult<WrappedAgentPubKey> {
    let agent_info = agent_info()?;

    Ok(WrappedAgentPubKey(agent_info.agent_initial_pubkey))
}

/// Create an expression and link it to yourself publicly with optional dna_address pointing to
/// dna that should ideally be used for linking any comments to this expression
#[hdk_extern]
pub fn create_public_expression(create_data: CreateExpression) -> ExternResult<ExpressionResponse> {
    Ok(ExpressionResponse(ExpressionDNA::create_public_expression(
        create_data.content,
    )?))
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
pub fn send_private(send_data: SendPrivate) -> ExternResult<StringResponse> {
    Ok(StringResponse(ExpressionDNA::send_private(
        send_data.to,
        send_data.content,
    )?))
}

/// Get private expressions sent to you optionally filtered by sender address
#[hdk_extern]
pub fn inbox(data: Inbox) -> ExternResult<ManyExpressionResponse> {
    Ok(ManyExpressionResponse(ExpressionDNA::inbox(
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
