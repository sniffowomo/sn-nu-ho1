use reth_primitives::{Address, Bytes, TransactionKind, H256};
use reth_rlp::Encodable;

/// EIP-2718 transaction type selector.
pub const DEPOSIT_TX_TYPE: u8 = 126;

/// A versioned byte sequence to enable the protocol to upgrade the deposit transaction type without
/// changing the transaction type selector.
pub const DEPOSIT_VERSION: u8 = 0;

/// Deposited transactions, also known as deposits are transactions which are initiated on L1, and
/// executed on L2. This document outlines a new transaction type for deposits. It also describes
/// how deposits are initiated on L1, along with the authorization and validation conditions on L2.
#[derive(Debug, Clone)]
pub struct TxDeposit {
    /// Hash that uniquely identifies the source of the deposit.
    pub source_hash: H256,
    /// The address of the sender account.
    pub from: Address,
    /// The address of the recipient account, or the null (zero-length) address if the deposited
    /// transaction is a contract creation.
    pub to: TransactionKind,
    /// The ETH value to mint on L2.
    pub mint: Option<u128>,
    ///  The ETH value to send to the recipient account.
    pub value: u128,
    /// The gas limit for the L2 transaction.
    pub gas_limit: u64,
    /// Field indicating if this transaction is exempt from the L2 gas limit.
    pub is_system_transaction: bool,
    /// Input has two uses depending if transaction is Create or Call (if `to` field is None or
    /// Some).
    pub input: Bytes,
}

impl TxDeposit {
    /// Outputs the length of the transaction's fields, without a RLP header or length of the
    /// eip155 fields.
    pub(crate) fn fields_len(&self) -> usize {
        let mut len = 0;
        len += self.source_hash.length();
        len += self.from.length();
        len += self.to.length();
        if let Some(mint) = self.mint {
            len += mint.length();
        }
        len += self.value.length();
        len += self.input.0.length();
        len += self.gas_limit.length();
        len += self.is_system_transaction.length();
        len
    }

    /// Encodes only the transaction's fields into the desired buffer, without a RLP header.
    /// https://github.com/ethereum-optimism/optimism/blob/develop/specs/deposits.md#the-deposited-transaction-type
    pub(crate) fn encode_fields(&self, out: &mut dyn bytes::BufMut) {
        self.source_hash.encode(out);
        self.from.encode(out);
        self.to.encode(out);
        if let Some(mint) = self.mint {
            mint.encode(out);
        }
        self.value.encode(out);
        self.input.encode(out);
        self.gas_limit.encode(out);
        self.is_system_transaction.encode(out);
    }
}
