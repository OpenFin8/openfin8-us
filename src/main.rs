// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the OpenFin8:US
// SPDX-License-Identifier: Apache-2.0

//! Sentin8 OpenFin8:US - US open banking: the FDX v6.x
//! provider/recipient wire. See `DESIGN.md`). This service
//! is the in-place successor of the retired bank8-fdx service.
//!
//! This is a THIN COUNTRY ADAPTER (the design's no-fork guarantee):
//! the entire implementation lives in `openfin8-core`, shared
//! byte-for-byte with `openfin8-ca`. This binary contributes
//! exactly three things: the US
//! [`Profile`](openfin8_core::profile::Profile) (jurisdiction
//! identity, scope vocabulary, schema `openfin8_us`, version from
//! `OPENFIN8_US_VERSION`), its registration data
//! (`registration.rs`), and its published-spec policy (`openapi.rs` -
//! OpenFin8-owned routes only; the FDX API specification is licensed and
//! never published here,).

use utoipa::OpenApi;

mod openapi;
mod registration;

use openfin8_core::profile::Profile;
use openfin8_core::server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let profile = Profile::united_states();
    sentin8::shell::serve(
        sentin8::shell::Sentin8Shell::new(profile.service_type, profile.env_prefix)
            .with_registration(registration::print)
            .with_state(|core| server::build_state(core, Profile::united_states()))
            .with_own_health()
            .with_router(server::routes)
            .with_openapi(openapi::UsOpenApi::openapi()),
    )
    .await
}
