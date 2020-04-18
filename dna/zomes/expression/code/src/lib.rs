#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

pub mod methods;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::cas::content::Address
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
    agent::AgentId,
    chain_header::ChainHeader
};

use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};
use hdk_proc_macros::zome;

/// An interface into a DNA which contains Expression information. Expected to be interacted with using expression Addresses 
/// retrieved from a social context or by using a Identity retreived from a users social graph. 
/// In this situation you can see that the Expression DNA/trait does not need to include any index capability
/// as this is already infered to the agent by the place they got the expression from; social context or social graph.
///
/// If the expression should be private to a group of people then the host DNA should be membraned.
pub trait ExpressionDao {
    /// Create an expression and link it to yourself publicly with optional dna_address pointing to 
    /// dna that should ideally be used for linking any comments to this expression
    fn create_public_expression(content: String, inter_dna_link_dna: Option<Address>) -> Expression;
    /// Get expressions authored by a given Agent/Identity
    fn get_by_author(author: Identity, count: u32, page: u32) -> Vec<Expression>;
    fn get_expression_by_address(address: Address) -> Option<Expression>;
    
    /// Send an expression to someone privately p2p
    fn send_private(to: Identity, content: String, inter_dna_link_dna: Option<Address>);
    /// Get private expressions sent to you
    fn inbox() -> Vec<Expression>;
}

pub type Identity = AgentId;

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct ShortFormExpression {
    background: Vec<String>,
    body: String
}

/// A holochain expression
struct Expression {
    entry: Entry,
    headers: Vec<ChainHeader>,
    expression_dna: Address,
    inter_dna_link_dna: Option<Address>,
}

#[zome]
pub mod shortform_expression {
    #[entry_def]
    pub fn group_entry_def() -> ValidatingEntryType {
        entry!(
            name: "shortform_expression",
            description: "ShortForm Expression Entry",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
    
            validation: | _validation_data: hdk::EntryValidationData<ShortFormExpression>| {
                Ok(())
            }
        )
    }

    #[init]
    pub fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }
}