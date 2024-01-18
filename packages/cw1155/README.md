# CW1155 Spec: Multiple Tokens

CW1155 is a specification for managing multiple tokens based on CosmWasm.
The name and design is poorly based on Ethereum's ERC1155 standard, with some 
modifications. 

A single contract instance allows to manage any combination of fungible and 
non-fungible tokens (e.g. semi-fungible tokens type can also be managed).

The types in here can be imported by contracts that wish to implement this 
spec, or by contracts that call to any standard CW1155 contract.

The specification is split into multiple sections, a contract may only
implement some of this functionality, but must implement the required 
interface.

## Specifications

### Base

This required interface o only handles ownership, transfers, and allowances.
These It must be supported as is by all CW1155 contracts.

#### Messages

- `TransferFrom{ from, to, id, amount, msg }` - Transfers some amount of 
tokens between two accounts. If `to` is an address controlled by a smart 
contract, it must implement the `CW1155Receiver` interface and `msg` will be 
passed to it along with other fields. Otherwise, `msg` should be `None`. The
operator should either be the `from` account or have approval from it.

- `BatchTransferFrom{ from, to, batch: Vec<( id, amount )>, msg }` - Batched 
version of `TransferFrom` which can handle multiple types of tokens at once.

- `ApproveAll{ operator, expires }` - Grants `operator` the permission to 
operate on all tokens owned by `env.sender`. This approval is tied to the 
owner, not the tokens and, while valid, it applies to any future token that 
the owner receives as well. If `expires` is set, this allowance has a 
time/height limit.

- `RevokeAll{ operator }` - Revokes `operator` a previously granted `ApproveAll` 
permission to operate on any token.

#### Queries

- `Balance{ owner, id }` - Queries the balance of `owner` on tokens of type `id`.

- `BatchBalance{ owner, ids }` - Batched version of `Balance`, queries the 
balance of `owner` on multiple token types.

- `IsApprovedForAll{ owner, operator }` - Queries if `operator` is currently 
granted to operate on all `owner`'s tokens.

#### Events

- `TransferSingle{ operator, from, to, id, amount }` - Is emitted when the `amount`
of tokens of type `id` are transferred from `from` to `to` by `operator`.

- `BatchTransfer{ operator, from, to, batch: Vec<( id, amount )> }` - Is equivalent
to multiple `TransferSingle` events, where `operator`, `from` and `to` are the 
same for all transfers.

- `ApprovedForAll{ account, operator, approved }` - Is emitted when `account` 
grants or revokes permission to `operator` to operate on its tokens. `approved`
is the bool tracking grant or revoke.

#### Receiver

Any contract wish to receive CW1155 tokens MUST implement 
`Cw1155ReceiveMsg` and `Cw1155BatchReceiveMsg`.
These are generally *not* implemented by any CW1155 contract.

For any message, the address of the sender contract is stored in `env.sender`
and cannot be faked. The receiver contract, on its side, should ensure the 
sender matches with the token contract it expects to handle.

- `Cw1155ReceiveMsg { operator, from, token_id, amount, msg }` - Is designed to
handle `TransferFrom` messages. The `operator` is the original account 
requesting to move the token and `msg` is a `Binary` data that can be decoded 
into a contract-specific message.

- `Cw1155BatchReceiveMsg { operator, from, batch: Vec<( id, amount )>, msg }` -
Is designed to handle batched transfer through the `BatchTransferFrom` 
messages. It works as the `Cw1155ReceiveMsg`.

## Extensions

### MetadataURI

#### Queries

- `Uri{ id }` - Queries the metadata uri of token of type `id`.

#### Events

- `Uri{ uri, id }` - Is emitted when the URI value for token type `id` changes.