// Copyright 2018 The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
#[macro_use]
extern crate bitflags;

pub mod blocks;
#[cfg(feature = "base_node")]
pub mod chain_storage;
pub mod consensus;
#[macro_use]
pub mod covenants;
#[cfg(feature = "base_node")]
pub mod iterators;
pub mod proof_of_work;
#[cfg(feature = "base_node")]
pub mod validation;

#[cfg(any(test, feature = "base_node"))]
#[macro_use]
pub mod test_helpers;

#[cfg(any(feature = "base_node", feature = "base_node_proto"))]
pub mod base_node;
#[cfg(any(feature = "base_node", feature = "base_node_proto"))]
pub mod proto;

#[cfg(any(feature = "base_node", feature = "mempool_proto"))]
pub mod mempool;

#[cfg(feature = "transactions")]
pub mod transactions;

mod common;
pub use common::{borsh, one_sided, ConfidentialOutputHasher};

#[cfg(feature = "base_node")]
mod domain_hashing {
    use blake2::Blake2b;
    use digest::consts::U32;
    use tari_crypto::{hash_domain, hashing::DomainSeparatedHasher};
    use tari_mmr::{
        pruned_hashset::PrunedHashSet,
        sparse_merkle_tree::SparseMerkleTree,
        BalancedBinaryMerkleTree,
        Hash,
        MerkleMountainRange,
    };

    hash_domain!(KernelMmrHashDomain, "com.tari.base_layer.core.kernel_mmr", 1);

    pub type KernelMmrHasherBlake256 = DomainSeparatedHasher<Blake2b<U32>, KernelMmrHashDomain>;
    pub type KernelMmr = MerkleMountainRange<KernelMmrHasherBlake256, Vec<Hash>>;
    pub type PrunedKernelMmr = MerkleMountainRange<KernelMmrHasherBlake256, PrunedHashSet>;

    hash_domain!(OutputSmtHashDomain, "com.tari.base_layer.core.output_smt", 1);
    pub type OutputSmtHasherBlake256 = DomainSeparatedHasher<Blake2b<U32>, OutputSmtHashDomain>;

    hash_domain!(InputMmrHashDomain, "com.tari.base_layer.core.input_mmr", 1);
    pub type InputMmrHasherBlake256 = DomainSeparatedHasher<Blake2b<U32>, InputMmrHashDomain>;
    pub type PrunedInputMmr = MerkleMountainRange<InputMmrHasherBlake256, PrunedHashSet>;

    pub type OutputSmt = SparseMerkleTree<OutputSmtHasherBlake256>;

    hash_domain!(
        ValidatorNodeBmtHashDomain,
        "com.tari.base_layer.core.validator_node_mmr",
        1
    );
    pub type ValidatorNodeBmtHasherBlake256 = DomainSeparatedHasher<Blake2b<U32>, ValidatorNodeBmtHashDomain>;
    pub type ValidatorNodeBMT = BalancedBinaryMerkleTree<ValidatorNodeBmtHasherBlake256>;
}

#[cfg(feature = "base_node")]
pub use domain_hashing::*;
