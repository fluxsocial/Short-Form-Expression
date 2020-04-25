use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::entry::Entry,
    holochain_persistence_api::cas::content::Address,
    holochain_persistence_api::hash::HashString,
    prelude::{
        GetEntryOptions, GetEntryResultType, GetLinksOptions, LinkMatch, Pagination,
        QueryArgsOptions, QueryResult, SizePagination, StatusRequestKind,
    },
    serde_json::json,
    AGENT_ADDRESS, DNA_ADDRESS,
};
use std::convert::TryFrom;

use crate::{Expression, ExpressionDao, ShortFormExpression, ShortFormExpressionWithSender};

impl ExpressionDao for Expression {
    /// Create an expression and link it to yourself publicly with optional dna_address pointing to
    /// dna that should ideally be used for linking any comments to this expression
    fn create_public_expression(content: String) -> ZomeApiResult<Expression> {
        // Serialize data to check its valid and prepare for entry into source chain
        let expression: ShortFormExpression = serde_json::from_str(&content)
            .map_err(|err| ZomeApiError::Internal(err.to_string()))?;
        let expression_entry = Entry::App("public_shortform_expression".into(), expression.into());
        // Commit and link entry
        let expression_address = hdk::commit_entry(&expression_entry)?;
        hdk::link_entries(&AGENT_ADDRESS, &expression_address, "", "")?;

        // Get headers for commited entry - is there a better way to do this such that headers are retrieved upon expression commit @Nico?
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
    fn get_by_author(
        author: Address,
        page_size: usize,
        page_number: usize,
    ) -> ZomeApiResult<Vec<Expression>> {
        let links = hdk::get_links_result(
            &author,
            LinkMatch::Any,
            LinkMatch::Any,
            GetLinksOptions {
                status_request: Default::default(),
                headers: false,
                timeout: Default::default(),
                pagination: Some(Pagination::Size(SizePagination {
                    page_number: page_number,
                    page_size: page_size,
                })),
                sort_order: None,
            },
            GetEntryOptions {
                status_request: StatusRequestKind::default(),
                entry: true,
                headers: true,
                timeout: Default::default(),
            },
        )?;

        Ok(links
            .into_iter()
            .map(|link| match link?.result {
                GetEntryResultType::Single(result) => Ok(Expression {
                    entry: result.entry.ok_or(ZomeApiError::Internal(String::from(
                        "Expected entry on link from identity",
                    )))?,
                    headers: result.headers,
                    expression_dna: HashString::from(DNA_ADDRESS.to_string()),
                }),
                _ => panic!("Should not hit this, right?"),
            })
            .collect::<ZomeApiResult<Vec<Expression>>>()?)
    }

    fn get_expression_by_address(address: Address) -> ZomeApiResult<Option<Expression>> {
        // Get headers and entry at given address
        match hdk::get_entry_result(
            &address,
            GetEntryOptions {
                status_request: StatusRequestKind::default(),
                entry: true,
                headers: true,
                timeout: Default::default(),
            },
        )?
        .result
        {
            GetEntryResultType::Single(result) => {
                if result.entry.is_some() {
                    Ok(Some(Expression {
                        entry: result.entry.unwrap(),
                        headers: result.headers,
                        expression_dna: HashString::from(DNA_ADDRESS.to_string()),
                    }))
                } else {
                    Ok(None)
                }
            }
            _ => panic!("Should not hit this, right?"),
        }
    }

    /// Send an expression to someone privately p2p
    fn send_private(to: Address, content: String) -> ZomeApiResult<String> {
        // Serialize data to check its valid before sending
        let _expression: ShortFormExpression = serde_json::from_str(&content)
            .map_err(|err| ZomeApiError::Internal(err.to_string()))?;
        hdk::send(to.into(), content, Default::default())
    }

    /// Get private expressions sent to you
    fn inbox(
        from: Option<Address>,
        page_size: usize,
        page_number: usize,
    ) -> ZomeApiResult<Vec<Expression>> {
        Ok(match from {
            Some(address) => {
                match hdk::query_result(
                    "private_shortform_expression".into(),
                    QueryArgsOptions {
                        start: page_number,
                        limit: page_size,
                        headers: true,
                        entries: true,
                    },
                )? {
                    QueryResult::HeadersWithEntries(results) => {
                        results
                            .into_iter()
                            .map(|results| {
                                (results.0, {
                                    match results.1 {
                                        Entry::App(_type, value) => {
                                            ShortFormExpressionWithSender::try_from(value)
                                                .expect("Entry could not be deserialized into a ShortFormExpressionWithSender")
                                        }
                                        _ => panic!("not reachable"),
                                    }
                                })
                            })
                            .filter(|i| i.1.sender == address)
                            .map(|results| Expression {
                                entry: Entry::App(
                                    "private_shortform_expression".into(),
                                    results.1.into(),
                                ),
                                headers: vec![results.0],
                                expression_dna: HashString::from(DNA_ADDRESS.to_string()),
                            })
                            .collect::<Vec<_>>()
                    }
                    _ => panic!("Should not hit this"),
                }
            }
            None => {
                match hdk::query_result(
                    "private_shortform_expression".into(),
                    QueryArgsOptions {
                        start: page_number,
                        limit: page_size,
                        headers: true,
                        entries: true,
                    },
                )? {
                    QueryResult::HeadersWithEntries(results) => results
                        .into_iter()
                        .map(|result| Expression {
                            entry: result.1,
                            headers: vec![result.0],
                            expression_dna: HashString::from(DNA_ADDRESS.to_string()),
                        })
                        .collect(),
                    _ => panic!("Should not hit this"),
                }
            }
        })
    }
}

pub fn handle_receive(from: Address, msg_json: String) -> String {
    let expression: Result<ShortFormExpression, _> = serde_json::from_str(&msg_json);
    json!({
        "msg_type": "response",
        "body": match expression {
            Ok(message) => {
                let message = ShortFormExpressionWithSender {
                    background: message.background,
                    body: message.body,
                    sender: from.clone()
                };
                // Some validation of payload here?
                let expression_entry = Entry::App("private_shortform_expression".into(), message.into()); 
                match hdk::commit_entry(&expression_entry) {
                    Ok(address) => match hdk::link_entries(&AGENT_ADDRESS, &address, "inbox", &from.to_string()) {
                        Ok(_result) => String::from("success"),
                        Err(err) => format!("error: {}", err)
                    },
                    Err(err) => format!("error: {}", err)
                }
            },
            Err(err) => format!("error: {}", err),
        }
    })
    .to_string()
}
