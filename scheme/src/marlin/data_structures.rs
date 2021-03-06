use math::PairingEngine;

use crate::Vec;

use crate::marlin::ahp::indexer::{Index, IndexInfo};
use crate::marlin::pc::{Commitment, CommitterKey, Proof as PCProof, Randomness, VerifierKey};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexVerifierKey<E: PairingEngine> {
    pub index_info: IndexInfo,
    pub index_comms: Vec<Commitment<E>>,
    pub verifier_key: VerifierKey<E>,
}

impl<E: PairingEngine> IndexVerifierKey<E> {
    pub fn iter(&self) -> impl Iterator<Item = &Commitment<E>> {
        self.index_comms.iter()
    }
}

impl<E: PairingEngine> math::ToBytes for IndexVerifierKey<E> {
    #[inline]
    fn write<W: math::io::Write>(&self, mut w: W) -> math::io::Result<()> {
        self.index_info.write(&mut w)?;
        (self.index_comms.len() as u32).write(&mut w)?;
        for i in &self.index_comms {
            i.write(&mut w)?;
        }
        self.verifier_key.write(&mut w)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexProverKey<'a, E: PairingEngine> {
    pub index: Index<'a, E::Fr>,
    pub index_rands: Vec<Randomness<E::Fr>>,
    pub index_verifier_key: IndexVerifierKey<E>,
    pub committer_key: CommitterKey<E>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proof<E: PairingEngine> {
    pub commitments: Vec<Vec<Commitment<E>>>,
    pub evaluations: Vec<E::Fr>,
    pub opening_proofs: Vec<PCProof<E>>,
}
