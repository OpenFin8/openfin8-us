// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the OpenFin8:US
// SPDX-License-Identifier: Apache-2.0

//! Library target for the openfin8-us adapter: the registration and
//! spec modules for the US profile. The implementation itself lives in
//! `openfin8-core` (shared with openfin8-ca -/4);
//! integration tests run from the core crate.

pub mod openapi;
pub mod registration;
