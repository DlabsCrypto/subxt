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

//! Create signed or unsigned extrinsics.
//!
//! This modules exposes the extrinsic's parameters and the ability to sign an extrinsic.
//!
//!
//! An extrinsic is submitted with an "signed extra" and "additional" parameters, which can be
//! different for each chain. The trait [ExtrinsicParams] determines exactly which
//! additional and signed extra parameters are used when constructing an extrinsic.
//!
//!
//! The structure [BaseExtrinsicParams] is a base implementation of the trait which
//! configures most of the "signed extra" and "additional" parameters as needed for
//! Polkadot and Substrate nodes. Only the shape of the tip payments differs, leading to
//! [SubstrateExtrinsicParams] and [PolkadotExtrinsicParams] structs which pick an
//! appropriate shape for Substrate/Polkadot chains respectively.

mod params;
mod signer;

pub use self::{
    params::{
        AssetTip,
        BaseExtrinsicParams,
        BaseExtrinsicParamsBuilder,
        Era,
        ExtrinsicParams,
        PlainTip,
        PolkadotExtrinsicParams,
        PolkadotExtrinsicParamsBuilder,
        SubstrateExtrinsicParams,
        SubstrateExtrinsicParamsBuilder,
    },
    signer::{
        PairSigner,
        Signer,
    },
};
