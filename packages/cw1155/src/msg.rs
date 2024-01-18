use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Binary, Uint128};
use cw_utils::Expiration;

#[cw_serde]
pub struct Token {
    pub id: String,
    pub amount: u128,
}

#[cw_serde]
pub enum Cw1155ExecuteMsg {
    /// allows to move tokens. It works if `env.sender` (the operator) either
    /// is the `from` account or has approval from it
    TransferFrom {
        /// the address from which take the tokens
        from: String,
        /// the address to which send the tokens. If it is not the address of a
        /// contract, the `msg` should be None
        to: String,
        /// the id of the token to transfer
        id: String,
        /// the quantity of the token to transfer. It must be lower or equal to
        /// the amount owned by the `from` address
        amount: Uint128,
        /// `None` means the `to` address is not a contract, or no particular
        /// call on the receiver smart contract interface
        msg: Option<Binary>,
    },

    /// allows to move multiple types of tokens in batch. As per the
    /// TransferFrom, it works if `env.sender` (the operator) either is the
    /// `from` account or has approval from it
    BatchTransferFrom {
        /// the address from which take the tokens
        from: String,
        /// the address to which send the tokens. If it is not the address of a
        /// contract, the `msg` should be None.
        to: String,
        /// the list of the ids of the tokens to transfer together with their
        /// amount, for each token type.
        batch: Vec<Token>,
        /// `None` means the `to` address is not a contract, or no particular
        /// call on the receiver smart contract interface
        msg: Option<Binary>,
    },

    /// grants `operator` the permission to operate on all tokens owned by
    /// `env.sender`. While valid, it applies to any future token that the
    /// owner receives as well. If `expires` is set, it has a height limit
    ApproveAll {
        /// the address of the operator which will be granted to manage the
        /// tokens
        operator: String,
        /// the time/height limit for the grant
        expires: Option<Expiration>,
    },

    /// removes a previously granted permission to operate on the
    /// `env.sender`'s tokens
    RevokeAll {
        /// the address of the operator which will no more be granted to manage
        /// the tokens
        operator: String,
    },
}
