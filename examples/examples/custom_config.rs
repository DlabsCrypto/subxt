// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! To run this example, a local polkadot node should be running. Example verified against polkadot 0.9.18-4542a603cc-aarch64-macos.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.18/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use sp_keyring::AccountKeyring;
use subxt::{
    ClientBuilder,
    Config,
    DefaultConfig,
    PairSigner,
    PolkadotExtrinsicParams,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

/// Custom [`Config`] impl where the default types for the target chain differ from the
/// [`DefaultConfig`]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MyConfig;
impl Config for MyConfig {
    // This is different from the default `u32`.
    //
    // *Note* that in this example it does differ from the actual `Index` type in the
    // polkadot runtime used, so some operations will fail. Normally when using a custom `Config`
    // impl types MUST match exactly those used in the actual runtime.
    type Index = u64;
    type BlockNumber = <DefaultConfig as Config>::BlockNumber;
    type Hash = <DefaultConfig as Config>::Hash;
    type Hashing = <DefaultConfig as Config>::Hashing;
    type AccountId = <DefaultConfig as Config>::AccountId;
    type Address = <DefaultConfig as Config>::Address;
    type Header = <DefaultConfig as Config>::Header;
    type Signature = <DefaultConfig as Config>::Signature;
    type Extrinsic = <DefaultConfig as Config>::Extrinsic;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<MyConfig, PolkadotExtrinsicParams<MyConfig>>>();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let hash = api
        .tx()
        .balances()
        .transfer(dest, 10_000)?
        .sign_and_submit_default(&signer)
        .await?;

    println!("Balance transfer extrinsic submitted: {}", hash);

    Ok(())
}
