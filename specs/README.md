# OpenFin8:US - Customer-Supplied Standard Specifications

**The FDX API specification is never committed to this repository.**

The FDX API specification is a subset of the Financial Data
Exchange ("FDX") API specification. Its usage (or any part of it)
constitutes acceptance of the FDX API License Agreement, which can
be found at <https://financialdataexchange.org/>. The FDX API
specification is distributed exclusively by FDX. Modifications that
eliminate required or conditional elements prescribed in the FDX
API Certification Use Cases will render implementations using
those modifications non-conformant with the FDX API Certification
Use Cases.

OpenFin8:US *implements* the FDX provider surface, but it does not
*publish* the specification. What this repository ships:

- `docs/openapi.json` - the bank8-owned surfaces only
  (`/v1/recipient/*`, `/v1/auth/consent/*`, `/v1/admin/*`,
  `/v1/health`, policy surfaces). The FDX resource paths are
  deliberately absent; a CI drift guard fails the build if any
  `/fdx/` path or FDX-derived schema ever appears in it.
- This service's behavior - the route tables, validation, and
  handler contracts - configured per environment via
  `OPENFIN8_US_VERSION`.

## Operators: supplying the specification

To obtain the FDX API specification for your deployment:

1. Become an FDX member or otherwise obtain the API specification
   under the FDX API License Agreement from
   <https://financialdataexchange.org/>.
2. Place the specification for the version your environment
   serves under this directory (gitignored):

   ```
   specs/fdx/6.5/openapi.yaml     # (or .json) - your licensed copy
   ```

3. Point your recipients, developer portal, and conformance tooling
   at your copy. Nothing in this service reads it - it is provided
   for your ecosystem's use.

The `specs/` directory is gitignored in this repository; committing
a licensed FDX specification here would violate the FDX API
License. CI enforces the published-spec half of this policy; the
not-committing half is an operator responsibility.