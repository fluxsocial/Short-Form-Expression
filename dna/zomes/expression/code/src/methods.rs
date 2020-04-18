use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::entry::Entry,
    holochain_persistence_api::cas::content::Address,
    holochain_persistence_api::hash::HashString,
    prelude::{GetEntryOptions, GetEntryResultType, StatusRequestKind},
    AGENT_ADDRESS, DNA_ADDRESS,
};

use crate::{Expression, ExpressionDao, Identity, ShortFormExpression};

impl ExpressionDao for Expression {
    /// Create an expression and link it to yourself publicly with optional dna_address pointing to
    /// dna that should ideally be used for linking any comments to this expression
    fn create_public_expression(content: String) -> ZomeApiResult<Expression> {
        // Serialize data to check its valid and prepare for entry into source chain
        let expression: ShortFormExpression = serde_json::from_str(&content)
            .map_err(|err| ZomeApiError::Internal(err.to_string()))?;
        let expression_entry = Entry::App("expression".into(), expression.into());

        // Commit and link entry
        let expression_address = hdk::commit_entry(&expression_entry)?;
        hdk::link_entries(&AGENT_ADDRESS, &expression_address, "", "")?;

        // Get headers for commited entry - is there a better way to do this @Nico?
        let entries_headers = match hdk::get_entry_result(
            &expression_address,
            GetEntryOptions {
                status_request: StatusRequestKind::default(),
                entry: false,
                headers: true,
                timeout: Default::default(),
            },
        )?
        .result
        {
            GetEntryResultType::Single(result) => result.headers,
            _ => vec![],
        };

        Ok(Expression {
            entry: expression_entry,
            headers: entries_headers,
            expression_dna: HashString::from(DNA_ADDRESS.to_string()),
        })
    }

    /// Get expressions authored by a given Agent/Identity
    fn get_by_author(author: Identity, count: u32, page: u32) -> Vec<Expression> {}
    fn get_expression_by_address(address: Address) -> Option<Expression> {}
    /// Send an expression to someone privately p2p
    fn send_private(to: Identity, content: String, inter_dna_link_dna: Option<Address>) {}
    /// Get private expressions sent to you
    fn inbox() -> Vec<Expression> {}
}
