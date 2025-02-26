//! PBH CTF starter bot
pub mod config;

use std::{path::PathBuf, pin::Pin, sync::Arc};

use alloy_primitives::Bytes;
use alloy_provider::{Provider, ProviderBuilder, WsConnect};
use alloy_signer_local::PrivateKeySigner;
use async_stream::stream;
use config::CTFConfig;
use eyre::eyre::Result;
use futures::{Stream, StreamExt};
use pbh_ctf::{
    CTFTransactionBuilder, Identity, PBH_CTF_CONTRACT, PBH_ENTRY_POINT,
    bindings::{IPBHEntryPoint::IPBHEntryPointInstance, IPBHKotH::IPBHKotHInstance},
    world_id::WorldID,
};
use reqwest::{Url, header};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbh_koth.toml");
    let config = CTFConfig::load(Some(config_path.as_path()))?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let signer = private_key.parse::<PrivateKeySigner>()?;
    let provider = Arc::new(
        ProviderBuilder::new()
            .on_ws(WsConnect::new(config.provider_uri.parse::<Url>()?))
            .await?,
    );

    // Initialize the WorldID
    let world_id = WorldID::new(&config.semaphore_secret)?;

    // TODO: get the latest pbh nonce number, explain why we have to do this

    // Initialize the King of the Hill contract
    let pbh_koth = IPBHKotHInstance::new(PBH_CTF_CONTRACT, provider.clone());
    let game_start = pbh_koth.latestBlock().call().await?._0;
    let game_end = pbh_koth.gameEnd().call().await?._0;

    // Initialize the PBHEntrypoint contract and get the PBH nonce limit
    let pbh_entrypoint = IPBHEntryPointInstance::new(PBH_ENTRY_POINT, provider.clone());
    let pbh_nonce_limit = pbh_entrypoint.numPbhPerMonth().call().await?._0;

    // TODO: Wait for the game to start

    // Subscribe to new blocks and prepare CTF transactions
    let mut block_stream = provider.subscribe_blocks().await?.into_stream();

    let player = signer.address();
    // TODO:
    let mut pbh_nonce = 0;
    while let Some(header) = block_stream.next().await {
        if header.number > game_end.to() {
            println!("The game has ended, thanks for playing!");
            break;
        }

        // If the user has not hit the pbh limit send a PBH tx, otherwise send a standard tx
        let tx = if pbh_nonce < pbh_nonce_limit {
            let calls = pbh_ctf::king_of_the_hill_multicall(player);
            CTFTransactionBuilder::new()
                .to(PBH_CTF_CONTRACT)
                .with_pbh_multicall(&world_id, pbh_nonce, calls)
                .await?
                .build(signer.clone())
                .await?
        } else {
            let calldata = pbh_ctf::king_of_the_hill_calldata(player);
            CTFTransactionBuilder::new()
                .to(PBH_CTF_CONTRACT)
                .input(calldata.into())
                .build(signer.clone())
                .await?
        };

        // TODO: send the transaction
    }

    Ok(())
}
