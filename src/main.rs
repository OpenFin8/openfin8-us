// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bank8:OpenUS
// SPDX-License-Identifier: Apache-2.0

//! Sentin8 Bank8:OpenUS - US open banking: the FDX v6.x
//! provider/recipient wire. See `DESIGN.md` (OBARCH-3). This service
//! is the in-place successor of `bank8-fdx`.
//!
//! This is a THIN COUNTRY ADAPTER (the design's no-fork guarantee):
//! the entire implementation lives in `bank8-open-core`, shared
//! byte-for-byte with `bank8-open-ca`. This binary contributes
//! exactly three things: the US
//! [`Profile`](bank8_open_core::profile::Profile) (jurisdiction
//! identity, scope vocabulary, schema `open_us`, version from
//! `BANK8_OPEN_US_VERSION`), its registration data
//! (`registration.rs`), and its published-spec policy (`openapi.rs` -
//! bank8-owned routes only; the FDX API specification is licensed and
//! never published here, OBARCH-2).

use utoipa::OpenApi;

mod openapi;
mod registration;

use bank8_open_core::profile::Profile;
use bank8_open_core::server;

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
