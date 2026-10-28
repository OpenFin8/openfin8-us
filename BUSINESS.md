# OpenFin8:US - Business Requirements & Functionality

> Audience: product owners, business analysts, and anyone evaluating what
> the US adapter **does for the business**. For architecture read
> `README.md`; for wire-level detail read `DESIGN.md` (OBARCH-3).

## 1. Business Purpose

OpenFin8:US is **US open banking**: the FDX v6.x wire for the United
States market - the in-place successor of the retired bank8-fdx
service. It delivers:

- **The identical served surface** as the Canadian adapter - accounts,
  transactions, statements, customers, consents, money movement,
  PIP, notifications, discovery, the recipient plane, and the hosted
  consent UI - because the implementation is shared byte-for-byte.
- **Independent US cadence** - `OPENFIN8_US_VERSION` and independent
  per-environment enablement: a bank operating both markets upgrades
  each on its own schedule.
- **US regulatory posture** - the CFPB-recognized FDX standard-setting
  body (Section 1033 ecosystem); the certification metrics serve US
  FDX certification review.
- **Zero-fork economics** - the US is a Profile + registration, not a
  port: market entry costs days of configuration, not quarters of
  engineering.

## 2. Business Scope (What the Service Provides)

Identical capability areas to the Canadian adapter (see
`openfin8-ca/BUSINESS.md` §2 for the complete 10-area table) - the
provider surface, consent surfaces, money movement, notifications,
discovery, recipient surfaces, hosted consent UI, admin console,
events (family `openfin8:us:*`), and health - with US identity:
service type `openfin8:us`, scopes `openfin8:us:*`, schema
`openfin8_us`, envs `OPENFIN8_US_*`.

**What the adapter does NOT own (business boundary):**

| Out of scope | Owned by |
|---|---|
| Consent ledger, registry, CSDF, notifications (the implementation) | openfin8-core (shared byte-for-byte with the CA adapter) |
| Account/transaction data | bank8:Ledger (public API only) |
| Money-movement execution | bank8:Payments |
| The OAuth ceremony hosting | Sentin8:Auth |
| The Canadian / UK surfaces | Their adapters (profile isolation) |

## 3. Business Capabilities

### 3.1 Why a separate service (not a config flag)

Three business reasons, even with an identical wire: independent
version cadence (US releases may diverge from Canadian), independent
enablement (a bank running both markets upgrades each independently
per environment), and regulatory posture (US certification and
compliance expectations get their own blast radius - never touching
another market's surface).

### 3.2 The identical-code guarantee

The US adapter contributes exactly three things: the US Profile
constructor (`Profile::united_states()`), its registration data, and
its published-spec policy. Everything else is the kernel. This is
auditable in the repository: `main.rs` + `registration.rs` +
`openapi.rs` and nothing else.

### 3.3 The served surface

The complete FDX Core Exchange subset: 71 routes across provider,
recipient, consent, admin, and internal surfaces; out-of-scope FDX
families return the standard unsupported response, never stubbed
data.

## 4. Business Events (Outbound)

Family `openfin8:us:*`, grammar
`openfin8:us:{resource}:{operation}` - the same 16 resource types as
the Canadian family, under the US identity.

## 5. Business Rules & Invariants

- Every consent/enforcement invariant from the kernel holds (no
  bypass, terminal revocation, fail-closed minimization).
- Version selection is per-environment configuration
  (`OPENFIN8_US_VERSION`), not a redeploy.
- The FDX API specification is licensed - **never published**; the
  published spec carries OpenFin8-owned routes only (CI
  drift-guarded); customers supply their licensed copy under
  gitignored `specs/`.
- This adapter mounts only US routes (profile isolation).

## 6. Target-User Roles & Use Cases

| Role | Use case |
|---|---|
| US bank (provider) | Enable `openfin8:us` in the bank's environment; serve the FDX surface; admin console for consents/audit/metrics |
| Fintech (recipient) | Discover US providers, run consent journeys, consume consented data |
| Bank customer | Authorizes on the hosted screen; reviews/revokes via the bank's app |
| Certification review | Reads the US certification-metrics evidence |

## 7. KPIs / Success Criteria

- **Market entry cost**: enabling the US market = enrollment +
  configuration (the Profile guarantee).
- **Zero implementation drift** from the Canadian adapter (auditable).
- **Spec policy compliance**: drift guard green on every CI run.
- **Certification readiness**: metrics current at query time.

## 8. Dependencies & Reliance on Other Services

Identical to the Canadian adapter: openfin8-core (the entire
implementation), bank8:Ledger/Payments (public APIs + `bank8::`
contracts - the OBVERT boundary), and the Sentin8 platform (Auth
ceremony, Bus events, Secrets custody, Storage statements, Sys
enablement, Flow steps).

## 9. Acceptance Criteria

- Build + tests green (publication guard included); CI green on the
  OpenFin8 org.
- The served surface matches the predecessor (bank8-fdx) under the US
  identity - verified during the migration.
- Independent enablement proven: enabling/disabling `openfin8:us`
  never affects an `openfin8:ca` deployment.

## 10. Open Business Questions

- Should a multi-major window ever be supported (serving `/fdx/v6`
  and `/fdx/v7` concurrently during a version migration), or does
  one-configured-version-per-env remain the model?
- Do US FDX releases (spring/fall cadence) warrant a per-version
  capability-announcement diff surfaced in the admin settings page?
- When payroll data structures enter scope (FDX open-finance
  direction), does the US adapter grow the payroll family over this
  kernel directly, or does a dedicated data-plane integration come
  first?

---

**Architecture & Design:** openfin8:us and its underlying concepts were
designed by [Hamid Jahanzad](https://www.linkedin.com/in/hamidjahanzad/).