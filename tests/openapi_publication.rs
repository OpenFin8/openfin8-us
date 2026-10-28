// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bank8:OpenUS
// SPDX-License-Identifier: Apache-2.0

//! OpenAPI publication drift guard (OBARCH-2).
//!
//! Two invariants:
//! 1. The published spec exists and carries the bank8-owned
//!    operation floor.
//! 2. The published spec contains NO standard-surface paths
//!    (the FDX API specification is licensed - it must never be
//!    published in this repository) and no FDX-derived schema
//!    names leak into components.

use serde_json::Value;

const STD_PATH_PREFIXES: &[&str] = &["/fdx/"];
const STD_SCHEMA_MARKERS: &[&str] = &[
    "FdxAccountsPassthrough",
    "FdxConsentIntrospection",
    "FdxConsentResource",
    "FdxCapability",
    "FdxAvailability",
];

#[test]
fn openapi_publication_policy_holds() {
    let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/openapi.json"))
        .expect("docs/openapi.json - regenerate via --print-registration --section=openapi");
    let spec: Value = serde_json::from_str(&text).expect("valid JSON");

    let paths = spec
        .get("paths")
        .and_then(Value::as_object)
        .expect("paths object");

    // Invariant 2a: no standard-surface paths.
    for path in paths.keys() {
        for prefix in STD_PATH_PREFIXES {
            assert!(
                !path.starts_with(prefix),
                "published spec must not carry standard-surface path {path} \
(FDX spec is licensed - see specs/README.md)"
            );
        }
    }

    // Invariant 2b: no standard-derived schema names in components.
    let schemas = spec
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    for name in schemas.keys() {
        for marker in STD_SCHEMA_MARKERS {
            assert!(
                name != marker,
                "published spec must not carry FDX-derived schema {name}"
            );
        }
    }

    // Invariant 1: bank8-owned operation floor (rises as the
    // core surfaces land).
    let ops: usize = paths
        .values()
        .map(|v| {
            v.as_object()
                .map(|o| o.keys().filter(|k| *k != "parameters").count())
                .unwrap_or(0)
        })
        .sum();
    assert!(
        ops >= 1,
        "spec floor: expected >= 1 bank8-owned op, got {ops}"
    );
}