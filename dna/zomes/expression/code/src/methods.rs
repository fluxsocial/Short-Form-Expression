use crate::{Expression, ExpressionDao};

impl ExpressionDao for Expression {
    /// Create an expression and link it to yourself publicly with optional dna_address pointing to 
    /// dna that should ideally be used for linking any comments to this expression
    fn create_public_expression(content: String, inter_dna_link_dna: Option<Address>) -> Expression {

    }
    /// Get expressions authored by a given Agent/Identity
    fn get_by_author(author: Identity, count: uint, page: uint) -> Vec<Expression> {

    }

    fn get_expression_by_address(address: Address) -> Option<Expression> {

    }
    
    /// Send an expression to someone privately p2p
    fn send_private(to: Identity, content: String, inter_dna_link_dna: Option<Address>) -> Result<(), ZomeApiError> {

    }
    /// Get private expressions sent to you
    fn inbox() -> Result<Vec<Expression>, ZomeApiError> {

    }
}