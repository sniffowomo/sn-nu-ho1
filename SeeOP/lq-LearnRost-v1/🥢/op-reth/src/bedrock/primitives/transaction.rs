use super::deposit::{TxDeposit, DEPOSIT_TX_TYPE, DEPOSIT_VERSION};
use reth_primitives::Transaction;
use reth_rlp::{length_of_length, Encodable, Header};

pub enum BedrockTransaction {
    /// Regular transaction wrapper variant.
    Regular(Transaction),
    /// Deposit transaction type.
    Deposit(TxDeposit),
}

impl BedrockTransaction {
    /// Get transaction type
    pub fn tx_type(&self) -> u8 {
        match self {
            Self::Regular(tx) => tx.tx_type() as u8,
            Self::Deposit(_) => DEPOSIT_TX_TYPE,
        }
    }
}

/// This encodes the transaction _without_ the signature, and is only suitable for creating a hash
/// intended for signing.
impl Encodable for BedrockTransaction {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        match self {
            Self::Regular(tx) => tx.encode(out),
            Self::Deposit(tx) => {
                out.put_u8(DEPOSIT_TX_TYPE);
                out.put_u8(DEPOSIT_VERSION);
                Header {
                    list: true,
                    payload_length: tx.fields_len(),
                }
                .encode(out);
                tx.encode_fields(out);
            }
        }
    }

    fn length(&self) -> usize {
        match self {
            Self::Regular(tx) => tx.length(),
            Self::Deposit(tx) => {
                let payload_length = tx.fields_len();
                // 'transaction type byte length' + 'version byte' + 'header length' + 'payload
                // length'
                1 + 1 + length_of_length(payload_length) + payload_length
            }
        }
    }
}
