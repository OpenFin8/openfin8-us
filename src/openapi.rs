// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bank8:OpenUS
// SPDX-License-Identifier: Apache-2.0

//! OpenAPI spec for Bank8:OpenUS - bank8-owned surfaces only.
//!
//! OBARCH-2: the FDX API specification is licensed under the FDX
//! API License Agreement (distributed exclusively by FDX) and is
//! NEVER published in this repository. `docs/openapi.json` (and
//! the served `/openapi.json`) therefore carry ONLY the bank8-
//! owned routes: `/v1/recipient/*`, `/v1/auth/consent/*`,
//! `/v1/admin/*`, `/v1/health`, and the policy surfaces. The
//! `/fdx/*` provider annotations aggregate into a separate,
//! non-published spec struct when the wire lands (OBARCH-3).
//!
//! The drift guard (`tests/openapi_publication.rs`) asserts this
//! by construction: zero `/fdx/` paths, zero FDX-derived schema
//! names.

use utoipa::OpenApi;

/// Bank8-owned OpenAPI surface for the US adapter.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "bank8-open-us",
        description = "Sentin8 Bank8:OpenUS - US open banking (FDX v6.x). \
The FDX API specification is licensed by FDX and is not published \
here; customers supply it under specs/fdx/{version}/ (see \
specs/README.md).",
        license(name = "MIT OR Apache-2.0", identifier = "MIT OR Apache-2.0"),
        version = "0.1.0"
    ),
    paths(),
    components(schemas()),
    security()
)]
pub struct UsOpenApi;
