use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::msg::Token;

#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw1155QueryMsg {
    /// returns the current balance of a given token type `id` for a given
    /// `owner` address
    /// Return type: BalanceResponse
    #[returns(BalanceResponse)]
    Balance {
        /// owner's address for which to query for the balance
        owner: String,
        /// token type id for the query
        id: String,
    },

    /// returns the batched version of `Balance`, querying on multiple tokens
    /// type id balances
    /// Return type: BatchBalanceResponse
    #[returns(BatchBalanceResponse)]
    BatchBalance {
        /// owner's address for which to query for the balances
        owner: String,
        /// list of token type ids for the query
        ids: Vec<String>,
    },

    /// returns if the `owner` granted the `operator` to manage all its tokens
    /// Return type: IsApprovedForAllResponse
    #[returns(IsApprovedForAllResponse)]
    IsApprovedForAll {
        /// tokens owner's address
        owner: String,
        /// operator which should be able to manage the owner's tokens
        operator: String,
    },

    /// With MetadataURI Extension.
    /// returns metadata URI for token type `id`
    /// Return type: URIResponse
    #[returns(URIResponse)]
    URI { id: String },
}

#[cw_serde]
pub struct BalanceResponse {
    /// value of the balance for the specific token type id
    pub balance: Token,
}

#[cw_serde]
pub struct BatchBalanceResponse {
    /// list of values of the balances for the specific token type ids
    pub balances: Vec<Token>,
}

#[cw_serde]
pub struct IsApprovedForAllResponse {
    /// if the user is granted to manage the owner's tokens
    pub approved: bool,
}

#[cw_serde]
pub struct URIResponse {
    /// should be a uri pointing to a json file following the related ERC1155
    /// Metadata JSON Schema
    pub uri: Option<String>,
}
