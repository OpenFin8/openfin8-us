// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the OpenFin8:US
// SPDX-License-Identifier: Apache-2.0

//! Service self-registration data for OpenFin8:US.
//! Output via `--print-registration` flag. See `sentin8/design/sentin8-service-self-registration-design.md`.

use sentin8::registration::{
    AdminScope, AdminUi, AdminUiPage, AuditFinding, ErrorCode, M2mGrant, PgRole, PolicySetDecl,
    Registration, RelationDecl, ServiceActionEntry, SqlMigration,
};
use utoipa::OpenApi;

const PG_ROLES: &[PgRole] = &[PgRole {
    scope: "openfin8:us:read,openfin8:us:write",
    role: "openfin8_us_service",
    bypass_rls: false,
    // Grants live in the openfin8 vertical database only (DBX §6.6
    // database-per-vertical, schema-per-service).
    database: "openfin8",
    // FDX owns all its reads - no cross-schema grants.
    cross_schema_grants: &[],
}];

/// M2M DCR scope grants (self-registration design §3.11). One entry per
/// workload this service registers via DCR; `vertical enroll` upserts
/// `certless:<workload>` rows into `sentin8.auth_m2m_scope_grants`. Scopes
/// use the full vocabulary form (bare forms + missing grants fail-closed
/// every bus/policy/stat DCR).
/// - `openfin8:us` - the service's own client (fdx/src/server.rs): bus
///
/// produce/consume (env-init `sentin8:sys:service:updated` consumer +
/// notifications).
/// - `openfin8:us-policy` - Shield hot-reload consumer
///
/// (policy_bus_client, full colon identity).
/// - `fdx-policy` - Shield stat emitter (stat_emit short form).
/// - `fdx-audit` - shell audit-journal pusher workload (audit crate
///
/// §6.8; boot step 8c).
const M2M_GRANTS: &[M2mGrant] = &[
    M2mGrant {
        workload: "openfin8:us",
        scopes: &[
            "sentin8:bus:produce",
            "sentin8:bus:consume",
            "sentin8:stat:measure",
            "sentin8:flow:write",
        ],
    },
    // The runtime DCR call sites pass the bare short name (`fdx` -
    // fdx/src/server.rs bus client + Flow registration, telemetry.rs's
    // stat client), not the colon identity. Grant BOTH spellings; the
    // gate looks up the exact client_name.
    M2mGrant {
        workload: "openfin8-us",
        scopes: &[
            "sentin8:bus:produce",
            "sentin8:bus:consume",
            "sentin8:stat:measure",
            "sentin8:flow:write",
        ],
    },
    // Shell audit-journal pusher workload (audit crate §6.8; boot step 8c).
    M2mGrant {
        workload: "fdx-audit",
        scopes: &["sentin8:sys:audit:write"],
    },
    M2mGrant {
        workload: "openfin8:us-policy",
        scopes: &["sentin8:bus:consume", "sentin8:stat:measure"],
    },
    M2mGrant {
        workload: "open-us-policy",
        scopes: &["sentin8:stat:measure"],
    },
    // tenant-snapshot API workload (design/sentin8-vertical-data-
    // plane-contract.md §VDC.5): the env-init consumer fetches tenant
    // identity (`GET /v1/tenants/{env_id}`) for issuer/callback derivation.
    M2mGrant {
        workload: "openfin8-us-sys",
        scopes: &["sentin8:sys:tenants:read"],
    },
    // CERTFREE-8V: the fdx→ledger proxy workload - the account-detail,
    // balances, and transaction reads the FDX surface proxies (the
    // static BANK8_LEDGER_TOKEN dev convention is retired; the ledger
    // side's scope-mirror policy set admits these scopes).
    M2mGrant {
        workload: "openfin8-us-ledger",
        scopes: &["bank8:ledger:accounts:read", "bank8:ledger:postings:read"],
    },
    // CERTFREE-8V: the fdx→payments proxy workload - transfers and
    // payment creation through the payments public API.
    M2mGrant {
        workload: "openfin8-us-payments",
        scopes: &[
            "bank8:payments:payments:read",
            "bank8:payments:payments:write",
        ],
    },
    // (design/sentin8-vertical-data-plane-contract.md §VDC.7):
    // the database-free HostContextResolver’s resolve workload - cache
    // miss calls Sys `GET /v1/resolve/hostname`.
    M2mGrant {
        workload: "openfin8-us-tls",
        scopes: &["sentin8:sys:instances:read"],
    },
];

const ADMIN_SCOPES: &[AdminScope] = &[
    AdminScope::Typed {
        role: "openfin8:us:admin",
        scope: "openfin8:us:consents:write",
    },
    AdminScope::Typed {
        role: "openfin8:us:admin",
        scope: "openfin8:us:consents:read",
    },
    AdminScope::Typed {
        role: "openfin8:us:admin",
        scope: "openfin8:us:recipient:read",
    },
    AdminScope::Typed {
        role: "openfin8:us:admin",
        scope: "openfin8:us:recipient:write",
    },
    AdminScope::Typed {
        role: "openfin8:us:admin",
        scope: "openfin8:us:provider:write",
    },
    AdminScope::Typed {
        role: "openfin8:us:admin",
        scope: "openfin8:us:provider:read",
    },
    // Policy-admin surface (policy design §3.6; principles §6.0b per-service scope).
    AdminScope::Typed {
        role: "openfin8:us:admin",
        scope: "openfin8:us:policy:read",
    },
    AdminScope::Typed {
        role: "openfin8:us:admin",
        scope: "openfin8:us:policy:write",
    },
];

const ERROR_CODES: &[ErrorCode] = &[
    ErrorCode {
        enum_name: "Fdx8ConsentInvalid",
        error_code: "fdx_consent_invalid",
        status_code: 403,
    },
    ErrorCode {
        enum_name: "Fdx8ConsentRevoked",
        error_code: "fdx_consent_revoked",
        status_code: 403,
    },
];

const ADMIN_UI_PAGES: &[AdminUiPage] = &[
    AdminUiPage {
        route: "overview",
        label: "Overview",
        ui_endpoint: "/v1/admin/ui/overview",
        data_endpoint: "/v1/admin/overview",
        icon: "0xe1b1",
    },
    AdminUiPage {
        route: "provider",
        label: "Provider",
        ui_endpoint: "/v1/admin/ui/provider",
        data_endpoint: "/v1/admin/provider",
        icon: "0xf06be",
    },
    AdminUiPage {
        route: "recipients",
        label: "Recipients",
        ui_endpoint: "/v1/admin/ui/recipients",
        data_endpoint: "/v1/admin/recipients",
        icon: "0xe2c8",
    },
    AdminUiPage {
        route: "consents",
        label: "Consents",
        ui_endpoint: "/v1/admin/ui/consents",
        data_endpoint: "/v1/admin/consents",
        icon: "0xe69a",
    },
    AdminUiPage {
        route: "csdf",
        label: "CSDF",
        ui_endpoint: "/v1/admin/ui/csdf",
        data_endpoint: "/v1/admin/csdf",
        icon: "0xf04e6",
    },
    AdminUiPage {
        route: "settings",
        label: "Settings",
        ui_endpoint: "/v1/admin/ui/settings",
        data_endpoint: "/v1/admin/settings",
        icon: "0xe57f",
    },
    // Sentin8:Policy admin surface (§3.6, post pattern).
    AdminUiPage {
        route: "policy-sets",
        label: "Policy Sets",
        ui_endpoint: "/v1/admin/ui/policy-sets",
        data_endpoint: "/v1/admin/policy-sets",
        icon: "0xe8b8",
    },
    AdminUiPage {
        route: "policy-decisions",
        label: "Policy Decisions",
        ui_endpoint: "/v1/admin/ui/policy-decisions",
        data_endpoint: "/v1/admin/policy-decisions",
        icon: "0xf070",
    },
    AdminUiPage {
        route: "relations",
        label: "Relations",
        ui_endpoint: "/v1/admin/ui/relations",
        data_endpoint: "/v1/admin/relations",
        icon: "0xe572",
    },
    AdminUiPage {
        route: "attributes",
        label: "Attributes",
        ui_endpoint: "/v1/admin/ui/attributes",
        data_endpoint: "/v1/admin/attributes",
        icon: "0xe8d3",
    },
];

/// Default declarative policy set (Sentin8:Policy design §3.2/§3.4,
/// seeded by `vertical enroll` as `origin='platform'`, `mode='enforced'`):
/// action-granular scope permits over the LIVE effective scope set - the
/// exact mirror of the gates this service's handlers evaluate in step 5
/// (policy design §4.3). Step 5 is the only
/// gate; shadow is the emergency one-UPDATE rollback lever only.
/// The Shield text of the declared scope-mirror set - public so the
/// policy wiring tests assert against the REAL seed.
pub const SCOPE_MIRROR_POLICIES: &str = "permit(principal, action == Action::\"openfin8:us:consents:read\", resource) when { principal.scopes.contains(\"openfin8:us:admin\") };\npermit(principal, action == Action::\"openfin8:us:consents:read\", resource) when { principal.scopes.contains(\"openfin8:us:consents:read\") };\npermit(principal, action == Action::\"openfin8:us:consents:read\", resource) when { principal.scopes.contains(\"openfin8:us:provider:read\") };\npermit(principal, action == Action::\"openfin8:us:consents:read\", resource) when { principal.scopes.contains(\"openfin8:us:recipient:read\") };\npermit(principal, action == Action::\"openfin8:us:consents:read\", resource) when { principal.scopes.contains(\"sentin8:sys:admin\") };\npermit(principal, action == Action::\"openfin8:us:consents:read\", resource) when { principal.scopes.contains(\"sentin8:sys:org:admin\") };\npermit(principal, action == Action::\"openfin8:us:consents:read\", resource) when { principal.scopes.contains(\"sentin8:sys:env:admin\") };\npermit(principal, action == Action::\"openfin8:us:consents:write\", resource) when { principal.scopes.contains(\"openfin8:us:consents:write\") };\npermit(principal, action == Action::\"openfin8:us:consents:write\", resource) when { principal.scopes.contains(\"sentin8:sys:admin\") };\npermit(principal, action == Action::\"openfin8:us:consents:write\", resource) when { principal.scopes.contains(\"sentin8:sys:org:admin\") };\npermit(principal, action == Action::\"openfin8:us:consents:write\", resource) when { principal.scopes.contains(\"sentin8:sys:env:admin\") };\npermit(principal, action == Action::\"openfin8:us:provider:write\", resource) when { principal.scopes.contains(\"openfin8:us:provider:write\") };\npermit(principal, action == Action::\"openfin8:us:provider:write\", resource) when { principal.scopes.contains(\"sentin8:sys:admin\") };\npermit(principal, action == Action::\"openfin8:us:provider:write\", resource) when { principal.scopes.contains(\"sentin8:sys:org:admin\") };\npermit(principal, action == Action::\"openfin8:us:provider:write\", resource) when { principal.scopes.contains(\"sentin8:sys:env:admin\") };\npermit(principal, action == Action::\"openfin8:us:recipient:read\", resource) when { principal.scopes.contains(\"openfin8:us:recipient:read\") };\npermit(principal, action == Action::\"openfin8:us:recipient:read\", resource) when { principal.scopes.contains(\"sentin8:sys:admin\") };\npermit(principal, action == Action::\"openfin8:us:recipient:read\", resource) when { principal.scopes.contains(\"sentin8:sys:org:admin\") };\npermit(principal, action == Action::\"openfin8:us:recipient:read\", resource) when { principal.scopes.contains(\"sentin8:sys:env:admin\") };\npermit(principal, action == Action::\"openfin8:us:recipient:write\", resource) when { principal.scopes.contains(\"openfin8:us:recipient:write\") };\npermit(principal, action == Action::\"openfin8:us:recipient:write\", resource) when { principal.scopes.contains(\"sentin8:sys:admin\") };\npermit(principal, action == Action::\"openfin8:us:recipient:write\", resource) when { principal.scopes.contains(\"sentin8:sys:org:admin\") };\npermit(principal, action == Action::\"openfin8:us:recipient:write\", resource) when { principal.scopes.contains(\"sentin8:sys:env:admin\") };";

const POLICIES: &[PolicySetDecl] = &[PolicySetDecl {
    policy_set_name: "scope-mirror",
    description: "Action-granular scope permits for openfin8:us over the LIVE effective scope set (design §3.1d). Step 5 is the only gate.",
    // Shield text lives in SCOPE_MIRROR_POLICIES (single source).
    policies: SCOPE_MIRROR_POLICIES,
    schema: None,
    mode: "enforced",
    default_enabled: true,
}];

/// Default ReBAC relation tuples (design §3.1c) - none declared by this
/// service today; env admins author their own graph via the relations
/// admin API (approver↔payment edges for payments, holder edges for
/// fdx consents are tenant-authored).
const RELATIONS: &[RelationDecl] = &[];

const SQL_1_INIT: &str = include_str!("../sql/init.sql");

// V2 convergence: provider_profiles is a PUBLIC cross-env registry (the
// recipient catalog) - the v1 blanket FORCE-RLS pass wrongly applied the
// tenant-visibility policies to it, so the cross-env catalog read saw
// nothing and the provider observation returned an empty list. The block
// strips the table's RLS (and any policies) idempotently; the v1 init.sql
// carries the same convergent statements for fresh clusters. The schema
// is open_ca (the service's own schema-per-service; sql/init.sql is the
// single schema source - the retired bank8-fdx schema name is gone).
const SQL_2_PROVIDER_CATALOG_NO_RLS: &str = r#"
ALTER TABLE open_ca.provider_profiles NO FORCE ROW LEVEL SECURITY;
ALTER TABLE open_ca.provider_profiles DISABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS provider_profiles_select ON open_ca.provider_profiles;
DROP POLICY IF EXISTS provider_profiles_insert ON open_ca.provider_profiles;
DROP POLICY IF EXISTS provider_profiles_update ON open_ca.provider_profiles;
DROP POLICY IF EXISTS provider_profiles_delete ON open_ca.provider_profiles;
"#;

const AUDIT_FINDINGS: &[AuditFinding] = &[
    AuditFinding {
        finding: "Integration credentials stored without a secret reference",
        recommendation: "Payments integrations, FDX provider profiles, and any third-party credential must be stored as a sentin8-secrets:// reference. The sentin8-secrets crate resolves the reference to the actual value at runtime.",
    },
];

const SERVICE_ACTIONS: &[ServiceActionEntry] = &[ServiceActionEntry {
    action: "fdx_consent_start",
    service_type: "openfin8:us",
    platform_domain_key: "openfin8:us",
    method: "POST",
    endpoint: "/v1/recipient/consents/start",
    scope: "openfin8:us:recipient:write",
}];

/// Consent data provider consumed by Sentin8:Auth's accounts proxy
/// (`WHERE feature='open-us' AND data_type='accounts'`, design §4.3b).
/// Field order matches the enroll INSERT contract (cli §vertical_registry).
fn consent_providers_json() -> Vec<serde_json::Value> {
    vec![serde_json::json!({
        "feature": "openfin8-us",
        "data_type": "accounts",
        "service_type": "openfin8:us",
        "platform_domain_key": "openfin8:us",
        "endpoint": "/v1/auth/consent/accounts",
        "scope": "openfin8:us:consents:read",
        "response_key": "accounts"
    })]
}

/// Consent flow declaring the FDX consent RFW surface (design §4.3b).
fn consent_flows_json() -> Vec<serde_json::Value> {
    vec![serde_json::json!({
        "id": "fdx_data_sharing",
        "trigger_patterns": ["openfin8:us:recipient:*"],
        "service_type": "openfin8:us",
        "ui_endpoint": "/v1/auth/consent/ui",
        "approve_endpoint": "/v1/auth/consent/approve",
        "deny_endpoint": "/v1/auth/consent/deny"
    })]
}

fn openapi_spec() -> serde_json::Value {
    serde_json::to_value(crate::openapi::UsOpenApi::openapi()).unwrap_or(serde_json::Value::Null)
}

/// Print the registration JSON to stdout.
pub fn print(section: Option<String>) -> anyhow::Result<()> {
    // Leak the two small JSON arrays to satisfy the `'static` field type
    // bounded, one-time allocation per process.
    let consent_providers: &'static [serde_json::Value] =
        Box::leak(consent_providers_json().into_boxed_slice());
    let consent_flows: &'static [serde_json::Value] =
        Box::leak(consent_flows_json().into_boxed_slice());
    let reg = Registration {
        service_type: "openfin8:us",
        subdomain: "openfin8-us",
        health_path: "/v1/health",
        admin_scopes: ADMIN_SCOPES,
        error_codes: ERROR_CODES,
        consent_providers,
        consent_flows,
        policies: POLICIES,
        relations: RELATIONS,
        event_triggers: &[],
        schedules: &[],
        admin_ui: Some(AdminUi {
            label: "OpenUS",
            icon: "0xf06be",
            service_flag: "openfin8-us",
            pages: ADMIN_UI_PAGES,
        }),
        sql_migrations: vec![
            SqlMigration {
                version: 1,
                name: "init",
                sql: SQL_1_INIT,
            },
            SqlMigration {
                version: 2,
                name: "provider_catalog_no_rls",
                sql: SQL_2_PROVIDER_CATALOG_NO_RLS,
            },
        ],
        audit_findings: Some(AUDIT_FINDINGS),
        service_actions: Some(SERVICE_ACTIONS),
        m2m_grants: Some(M2M_GRANTS),
        pg_roles: PG_ROLES,
        openapi_url: "/openapi.json",
        openapi: openapi_spec,
    };
    sentin8::registration::print_registration(section, &reg, openapi_spec)
}
