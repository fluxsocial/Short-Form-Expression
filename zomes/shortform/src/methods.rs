use hdk3::host_fn::zome_info::zome_info;
use hdk3::prelude::*;
use meta_traits::{Expression, ExpressionDao, Identity};

use crate::{inputs::CreatePrivateExpression, utils::err};
use crate::{ExpressionDNA, PrivateAgent, PrivateShortFormExpression, ShortFormExpression};

impl ExpressionDao for ExpressionDNA {
    /// Create an expression and link it to yourself publicly
    fn create_public_expression(content: String) -> ExternResult<Expression> {
        // Serialize data to check its valid and prepare for entry into source chain
        let expression: ShortFormExpression = serde_json::from_str(&content)
            .map_err(|_| err("Could not serialized content into ShortForm expression type"))?;
        let expression_hash = hash_entry(&expression)?;
        create_entry(&expression)?;

        //Here we probably want to do some path stuff where we get all their pub keys and make sure they are linked together so getting by author later
        //can use all used agent pub keys
        create_link(
            agent_info()?.agent_latest_pubkey.into(),
            expression_hash.clone(),
            LinkTag::new("expression".as_bytes().to_owned()),
        )?;

        Ok(Expression {
            expression_dna: zome_info()?.dna_hash,
            expression: get(expression_hash, GetOptions::default())?
                .ok_or(err("Could not get entry after commit"))?,
        })
    }

    /// Get expressions authored by a given Agent/Identity
    fn get_by_author(
        author: Identity,
        //For now we are ignoring these page values as pagination is not implemented on get_links. But perhaps this could be used in some chunking pattern?
        _page_size: usize,
        _page_number: usize,
    ) -> ExternResult<Vec<Expression>> {
        //TODO: try and get all pub keys that this target agent has used and get all links from each pub key
        let links = get_links(
            author.into(),
            Some(LinkTag::new("expression".as_bytes().to_owned())),
        )
        .map_err(|_| err("Could not get links on author"))?;
        let dna_hash = zome_info()?.dna_hash;
        Ok(links
            .into_inner()
            .into_iter()
            .map(|link| {
                Ok(Expression {
                    expression_dna: dna_hash.clone(),
                    expression: get(link.target, GetOptions::default())?
                        .ok_or(err("Could not get entry after commit"))?,
                })
            })
            .collect::<Result<Vec<Expression>, HdkError>>()?)
    }

    fn get_expression_by_address(address: AnyDhtHash) -> ExternResult<Option<Expression>> {
        let expression = get(address, GetOptions::default())?;
        match expression {
            Some(expression) => Ok(Some(Expression {
                expression_dna: zome_info()?.dna_hash,
                expression: expression,
            })),
            None => Ok(None),
        }
    }

    /// Send an expression to someone privately p2p
    fn send_private(to: Identity, content: String) -> ExternResult<String> {
        // Serialize data to check its valid
        let _expression: PrivateShortFormExpression = serde_json::from_str(&content)
            .map_err(|_| err("Could not serialized content into ShortForm expression type"))?;
        let create_exp_data = CreatePrivateExpression {
            content: content.clone(),
            sender: agent_info()?.agent_latest_pubkey,
        };

        //Call the users remote zome
        //TODO here we want some pattern better than this; only having this succeed when agent is online is not great
        //Here I am sending the identity of the callee of this fn since I dont know if we can get this information in recv_private_expression?
        //Id imagine there is some way but for now this can work fine...
        call_remote(
            to,
            ZomeName::from("shortform"),
            FunctionName::from("recv_private_expression"),
            None,
            &create_exp_data,
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

        Ok(content)
    }

    /// Get private expressions sent to you optionally filtered by sender address
    fn inbox(
        from: Option<Identity>,
        _page_size: usize,
        _page_number: usize,
    ) -> ExternResult<Vec<Expression>> {
        match from {
            Some(ident) => {
                let links = get_links(
                    hash_entry(&PrivateAgent(ident.into()))?,
                    Some(LinkTag::new("expression".as_bytes().to_owned())),
                )
                .map_err(|_| err("Could not get links on author"))?;
                let dna_hash = zome_info()?.dna_hash;
                Ok(links
                    .into_inner()
                    .into_iter()
                    .map(|link| {
                        Ok(Expression {
                            expression_dna: dna_hash.clone(),
                            expression: get(link.target, GetOptions::default())?
                                .ok_or(err("Could not get entry after commit"))?,
                        })
                    })
                    .collect::<Result<Vec<Expression>, HdkError>>()?)
            }
            None => {
                let priv_exp_entry_def = PrivateShortFormExpression::entry_def();
                //Not sure about the entrytype here...
                let query = query(QueryFilter::new().entry_type(EntryType::App(
                    AppEntryType::new(1.into(), 0.into(), priv_exp_entry_def.visibility),
                )))?;
                let dna_hash = zome_info()?.dna_hash;
                Ok(query
                    .0
                    .into_iter()
                    .map(|elem| {
                        Ok(Expression {
                            expression_dna: dna_hash.clone(),
                            expression: elem,
                        })
                    })
                    .collect::<Result<Vec<Expression>, HdkError>>()?)
            }
        }
    }
}

pub fn recv_private_expression(create_data: CreatePrivateExpression) -> ExternResult<()> {
    let agent_entry = PrivateAgent(create_data.sender);
    let agent_entry_hash = hash_entry(&agent_entry)?;
    create_entry(&agent_entry)?;

    // Serialize data to check its valid and prepare for entry into source chain
    let expression_entry: PrivateShortFormExpression =
        serde_json::from_str(&create_data.content)
            .map_err(|_| err("Could not serialized content into ShortForm expression type"))?;
    let expression_entry_hash = hash_entry(&expression_entry)?;
    create_entry(&expression_entry)?;

    create_link(
        agent_entry_hash,
        expression_entry_hash,
        LinkTag::from("expression".as_bytes().to_owned()),
    )?;

    Ok(())
}
