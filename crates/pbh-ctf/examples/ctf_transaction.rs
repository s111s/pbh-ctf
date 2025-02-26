use alloy_network::{EthereumWallet, TransactionBuilder};
use alloy_primitives::Address;
use alloy_primitives::TxKind;
use alloy_rpc_types_eth::{TransactionInput, TransactionRequest};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::SolCall;
use alloy_sol_types::SolValue;
use base64::{Engine, engine::general_purpose};
use pbh_ctf::CTFTransactionBuilder;

use pbh_ctf::PBH_CTF_CONTRACT;
use world_chain_builder_pbh::payload::PBHPayload;
use world_chain_builder_test_utils::{
    WC_SEPOLIA_CHAIN_ID,
    bindings::{IMulticall3, IPBHEntryPoint},
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // The private key of the transaction signer
    let signer = std::env::var("PRIVATE_KEY")?.parse::<PrivateKeySigner>()?;

    let player = Address::random();
    let calldata = pbh_ctf::king_of_the_hill_calldata(player);

    let _tx = CTFTransactionBuilder::new()
        .to(PBH_CTF_CONTRACT)
        .input(calldata.into())
        .build(signer)
        .await?;

    Ok(())
}
