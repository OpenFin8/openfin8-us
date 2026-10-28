// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bank8:OpenUS
// SPDX-License-Identifier: Apache-2.0

//! Sentin8 Bank8:OpenUS - US open banking, FDX v6.x provider/recipient.
//! See `DESIGN.md` (OBARCH-3).
//!
//! This service is the in-place successor of `bank8-fdx`: it embeds
//! `bank8-open-core` (the jurisdiction-agnostic kernel - OBARCH-1)
//! and supplies the US FDX profile, the FDX ceremony (DCR + PAR +
//! PKCE + DPoP), and the FDX provider router (`/fdx/v{major}/*`
//! from the configured `BANK8_OPEN_US_VERSION`).
//!
//! Skeleton - the OBARCH-1 core extraction lands first; this
//! binary then takes the extracted FDX wire handlers, the version-
//! parameterized route contract, and the renamed registration
//! data. The FDX spec is NEVER published here (OBARCH-2): the
//! published `docs/openapi.json` carries bank8-owned routes only;
//! customers supply the licensed FDX API specification under
//! gitignored `specs/fdx/{version}/` (see `specs/README.md`).

use utoipa::OpenApi;

mod openapi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    sentin8::shell::serve(
        sentin8::shell::Sentin8Shell::new("bank8:open-us", "BANK8_OPEN_US")
            .with_registration(print_registration_stub)
            .with_own_health()
            .with_openapi(openapi::UsOpenApi::openapi()),
    )
    .await
}

/// Registration stub - the full registration data (policy sets,
/// Shield gates `bank8:open-us:*`, admin UI pages, SQL migrations)
/// lands with the OBARCH-1 extraction; the shape follows the
/// `bank8-fdx` registration (renamed).
fn print_registration_stub(section: Option<String>) -> anyhow::Result<()> {
    // Placeholder until the registration module lands (OBARCH-3).
    let _ = section;
    Ok(())
}
