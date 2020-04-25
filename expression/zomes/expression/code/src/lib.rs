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

use hdk::holochain_core_types::{
    chain_header::ChainHeader, dna::entry_types::Sharing, entry::Entry,
};
use hdk::{
    entry_definition::ValidatingEntryType, error::ZomeApiResult,
    holochain_persistence_api::cas::content::Address,
};

use hdk::holochain_json_api::{error::JsonError, json::JsonString};
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
    fn create_public_expression(content: String) -> ZomeApiResult<Expression>;
    /// Get expressions authored by a given Agent/Identity
    fn get_by_author(
        author: Address,
        page_size: usize,
        page_number: usize,
    ) -> ZomeApiResult<Vec<Expression>>;
    fn get_expression_by_address(address: Address) -> ZomeApiResult<Option<Expression>>;

    /// Send an expression to someone privately p2p
    fn send_private(to: Address, content: String) -> ZomeApiResult<String>;
    /// Get private expressions sent to you optionally filtered by sender address
    fn inbox(
        from: Option<Address>,
        page_size: usize,
        page_number: usize,
    ) -> ZomeApiResult<Vec<Expression>>;
}

/// Expression data this DNA is hosting
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ShortFormExpression {
    background: Vec<String>,
    body: String,
}

/// Expression data this DNA is hosting
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ShortFormExpressionWithSender {
    background: Vec<String>,
    body: String,
    sender: Address,
}

/// A holochain expression
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Expression {
    entry: Entry,
    headers: Vec<ChainHeader>,
    expression_dna: Address,
}

#[zome]
pub mod shortform_expression {
    #[entry_def]
    pub fn expression_entry_def() -> ValidatingEntryType {
        entry!(
            name: "public_shortform_expression",
            description: "Public ShortForm Expression Entry",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: | _validation_data: hdk::EntryValidationData<ShortFormExpression>| {
                Ok(())
            },

            links: [
                from!(
                    "%agent_id",
                    link_type: "",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData | {
                        Ok(())
                    }
                )
            ]
        )
    }

    #[entry_def]
    pub fn private_expression_entry_def() -> ValidatingEntryType {
        entry!(
            name: "private_shortform_expression",
            description: "Private ShortForm Expression Entry",
            sharing: Sharing::Private,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: | _validation_data: hdk::EntryValidationData<ShortFormExpressionWithSender>| {
                Ok(())
            },

            links: [
                from!(
                    "%agent_id",
                    link_type: "inbox",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData | {
                        Ok(())
                    }
                )
            ]
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

    #[receive]
    pub fn receive(from: Address, msg_json: String) {
        methods::handle_receive(from, msg_json)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("expression")]
    pub fn create_public_expression(content: String) -> ZomeApiResult<Expression> {
        Expression::create_public_expression(content)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("expression")]
    pub fn get_by_author(
        author: Address,
        page_size: usize,
        page_number: usize,
    ) -> ZomeApiResult<Vec<Expression>> {
        Expression::get_by_author(author, page_size, page_number)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("expression")]
    pub fn get_expression_by_address(address: Address) -> ZomeApiResult<Option<Expression>> {
        Expression::get_expression_by_address(address)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("expression")]
    pub fn send_private(to: Address, content: String) -> ZomeApiResult<String> {
        Expression::send_private(to, content)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("expression")]
    pub fn inbox(
        from: Option<Address>,
        page_size: usize,
        page_number: usize,
    ) -> ZomeApiResult<Vec<Expression>> {
        Expression::inbox(from, page_size, page_number)
    }
}
