use hdk3::prelude::*;

use crate::{inputs::CreateExpression, outputs::HolochainData, utils::err};
use crate::{
    AcaiAgent, ExpressionDNA, ExpressionResponse, PrivateAcaiAgent, PrivateExpressionResponse,
    PrivateShortFormExpression, ShortFormExpression,
};

impl ExpressionDNA {
    /// Create an expression and link it to yourself publicly
    pub fn create_public_expression(content: CreateExpression) -> ExternResult<ExpressionResponse> {
        // Serialize data to check its valid and prepare for entry into source chain
        let expression = ShortFormExpression::try_from(content)?;
        let expression_hash = hash_entry(&expression)?;
        create_entry(&expression)?;

        let acai_agent = AcaiAgent(expression.author.did.clone());
        let acai_agent_hash = hash_entry(&acai_agent)?;
        create_entry(&acai_agent)?;

        //Here we will want to use chunking mixin
        create_link(acai_agent_hash, expression_hash.clone(), LinkTag::new("expression"))?;

        let expression_element = get(expression_hash, GetOptions::default())?
            .ok_or(err("Could not get entry after commit"))?;
        let timestamp = expression_element.header().timestamp();

        Ok(ExpressionResponse {
            expression_data: expression,
            holochain_data: HolochainData {
                element: expression_element,
                expression_dna: zome_info()?.dna_hash,
                creator: agent_info()?.agent_latest_pubkey,
                created_at: chrono::DateTime::from_utc(
                    chrono::naive::NaiveDateTime::from_timestamp(timestamp.0, timestamp.1),
                    chrono::Utc,
                ),
            },
        })
    }

    /// Get expressions authored by a given Agent/Identity
    pub fn get_by_author(
        author: String,
        //These pages can be input into chunking mixin for the future
        _page_size: usize,
        _page_number: usize,
    ) -> ExternResult<Vec<ExpressionResponse>> {
        let links = get_links(
            hash_entry(&AcaiAgent(author))?,
            Some(LinkTag::new("expression")),
        )
        .map_err(|_| err("Could not get links on author"))?;

        Ok(links
            .into_inner()
            .into_iter()
            .map(|link| {
                let expression_element = get(link.target, GetOptions::default())?
                    .ok_or(err("Could not get entry after commit"))?;
                let timestamp = expression_element.header().timestamp();
                let exp_data: ShortFormExpression = expression_element
                    .entry()
                    .to_app_option()?
                    .ok_or(HdkError::Wasm(WasmError::Zome(String::from(
                        "Could not deserialize link expression data into ShortFormExpression",
                    ))))?;
                Ok(ExpressionResponse {
                    expression_data: exp_data,
                    holochain_data: HolochainData {
                        element: expression_element,
                        expression_dna: zome_info()?.dna_hash,
                        creator: agent_info()?.agent_latest_pubkey,
                        created_at: chrono::DateTime::from_utc(
                            chrono::naive::NaiveDateTime::from_timestamp(timestamp.0, timestamp.1),
                            chrono::Utc,
                        ),
                    },
                })
            })
            .collect::<Result<Vec<ExpressionResponse>, HdkError>>()?)
    }

    pub fn get_expression_by_address(
        address: AnyDhtHash,
    ) -> ExternResult<Option<ExpressionResponse>> {
        let expression = get(address, GetOptions::default())?;
        match expression {
            Some(expression_element) => {
                let exp_data: ShortFormExpression = expression_element
                    .entry()
                    .to_app_option()?
                    .ok_or(HdkError::Wasm(WasmError::Zome(String::from(
                        "Could not deserialize link expression data into ShortFormExpression",
                    ))))?;
                let timestamp = expression_element.header().timestamp();
                Ok(Some(ExpressionResponse {
                    expression_data: exp_data,
                    holochain_data: HolochainData {
                        element: expression_element,
                        expression_dna: zome_info()?.dna_hash,
                        creator: agent_info()?.agent_latest_pubkey,
                        created_at: chrono::DateTime::from_utc(
                            chrono::naive::NaiveDateTime::from_timestamp(timestamp.0, timestamp.1),
                            chrono::Utc,
                        ),
                    },
                }))
            }
            None => Ok(None),
        }
    }

    /// Send an expression to someone privately p2p
    pub fn send_private(
        to: AgentPubKey,
        expression: CreateExpression,
    ) -> ExternResult<PrivateShortFormExpression> {
        // Serialize data to check its valid
        let expression = ShortFormExpression::try_from(expression)?;
        let expression = PrivateShortFormExpression::from(expression);

        //Call the users remote zome
        //TODO here we want some pattern better than this; only having this succeed when agent is online is not great
        //Here I am sending the identity of the callee of this fn since I dont know if we can get this information in recv_private_expression?
        //Id imagine there is some way but for now this can work fine...
        call_remote(
            to,
            ZomeName::from("shortform"),
            FunctionName::from("recv_private_expression"),
            None,
            &expression,
        )
        .map_err(|error| match error {
            HdkError::UnauthorizedZomeCall(_, _, _, _) => {
                err("This agent is not allowing private messages")
            }
            HdkError::ZomeCallNetworkError(_) => {
                err("Unable to send message now; likely that this agent is offline")
            }
            _ => err(format!("{:?}", error).as_ref()),
        })?;

        Ok(expression)
    }

    /// Get private expressions sent to you optionally filtered by sender address
    pub fn inbox(
        from: Option<String>,
        _page_size: usize,
        _page_number: usize,
    ) -> ExternResult<Vec<PrivateExpressionResponse>> {
        match from {
            Some(ident) => {
                let links = get_links(
                    hash_entry(&PrivateAcaiAgent(ident.clone().into()))?,
                    Some(LinkTag::new("expression")),
                )?;

                Ok(links
                    .into_inner()
                    .into_iter()
                    .map(|link| {
                        let expression_element = get(link.target, GetOptions::default())?
                            .ok_or(err("Could not get entry after commit"))?;
                        let timestamp = expression_element.header().timestamp();
                        let exp_data: PrivateShortFormExpression = expression_element
                            .entry()
                            .to_app_option()?
                            .ok_or(HdkError::Wasm(WasmError::Zome(String::from(
                                "Could not deserialize link expression data into PrivateShortFormExpression",
                            ))))?;
                        Ok(PrivateExpressionResponse {
                            expression_data: exp_data,
                            holochain_data: HolochainData {
                                element: expression_element,
                                expression_dna: zome_info()?.dna_hash,
                                creator: agent_info()?.agent_latest_pubkey,
                                created_at: chrono::DateTime::from_utc(
                                    chrono::naive::NaiveDateTime::from_timestamp(timestamp.0, timestamp.1),
                                    chrono::Utc,
                                ),
                            },
                        })
                    })
                    .collect::<Result<Vec<PrivateExpressionResponse>, HdkError>>()?)
            }
            None => {
                let priv_exp_entry_def = PrivateShortFormExpression::entry_def();
                //Not sure about the entrytype here...
                let query = query(QueryFilter::new().entry_type(EntryType::App(
                    AppEntryType::new(1.into(), 0.into(), priv_exp_entry_def.visibility),
                )).include_entries(true))?;
                Ok(query
                    .0
                    .into_iter()
                    .map(|expression_element| {
                        let exp_data: PrivateShortFormExpression = expression_element
                            .entry()
                            .to_app_option()?
                            .ok_or(HdkError::Wasm(WasmError::Zome(String::from(
                                "Could not deserialize local expression data into PrivateShortFormExpression",
                            ))))?;
                        let timestamp = expression_element.header().timestamp();
                        Ok(PrivateExpressionResponse {
                            expression_data: exp_data,
                            holochain_data: HolochainData {
                                element: expression_element,
                                expression_dna: zome_info()?.dna_hash,
                                creator: agent_info()?.agent_latest_pubkey,
                                created_at: chrono::DateTime::from_utc(
                                    chrono::naive::NaiveDateTime::from_timestamp(timestamp.0, timestamp.1),
                                    chrono::Utc,
                                ),
                            },
                        })
                    })
                    .collect::<Result<Vec<PrivateExpressionResponse>, HdkError>>()?)
            }
        }
    }

    pub fn recv_private_expression(create_data: PrivateShortFormExpression) -> ExternResult<()> {
        let agent_entry = PrivateAcaiAgent(create_data.author.did.clone());
        let agent_entry_hash = hash_entry(&agent_entry)?;
        create_entry(&agent_entry)?;

        let expression_entry_hash = hash_entry(&create_data)?;
        create_entry(&create_data)?;

        create_link(
            agent_entry_hash,
            expression_entry_hash,
            LinkTag::new("expression"),
        )?;

        Ok(())
    }
}
