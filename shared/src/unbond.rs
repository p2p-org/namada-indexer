use fake::Fake;

use crate::balance::Amount;
use crate::block::Epoch;
use crate::id::Id;

// TODO: maybe reuse bond with Option<Amount> instead of Amount
#[derive(Debug, Clone)]
pub struct UnbondAddresses {
    pub source: Id,
    pub validator: Id,
}

#[derive(Debug, Clone)]
pub struct Unbond {
    pub source: Id,
    pub target: Id,
    pub amount: Amount,
    pub withdraw_at: Epoch,
}

#[derive(Debug, Clone)]
pub struct Unbonds {
    pub epoch: Epoch,
    pub values: Vec<Unbond>,
}

impl Unbond {
    pub fn fake(validator_address: Id) -> Self {
        let source_address =
            namada_core::address::gen_established_address("namada-indexer");

        Self {
            source: Id::Account(source_address.to_string()),
            target: validator_address,
            amount: Amount::fake(),
            withdraw_at: (3..10).fake::<u32>(),
        }
    }
}

impl Unbonds {
    pub fn new(unbonds: Vec<Unbond>, epoch: Epoch) -> Self {
        Self {
            epoch,
            values: unbonds,
        }
    }
}
