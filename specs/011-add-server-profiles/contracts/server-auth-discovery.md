# Contract: Server Auth Discovery And Login

## Purpose

Defines the server-facing contract used by the multi-instance client when adding or activating a remote profile.

## Discovery

### Request

`GET /api/auth/mode`

No authentication required.

### Required Response

| Field | Type | Meaning |
|---|---|---|
| `mode` | string | Server auth mode |

### Optional Additive Response Fields

| Field | Type | Meaning |
|---|---|---|
| `server_id` | string | Stable identity for this server instance |
| `server_name` | string | Human-readable server label |
| `api_version` | string | Compatibility indicator for the client |

### Accepted Modes For This Feature

| Mode | Client Behavior |
|---|---|
| `local` | Allowed only for a local workspace profile; not used for remote username/password sign-in |
| `secret_key` | Supported remote mode for this feature |
| `oidc` | Treated as unsupported for this feature's remote profile flow |

### Failure Classification

| Failure | Expected Client Handling |
|---|---|
| Server unreachable / timeout | Keep profile unsaved or marked unreachable; show retry/edit path |
| Non-Nyx or invalid response | Mark as invalid server; do not continue to login |
| Unsupported mode | Mark as unsupported for this feature; do not continue to login |

## Login

### Request

`POST /api/auth/login`

Body:

```json
{
  "username": "alice",
  "password": "correct horse battery staple"
}
```

### Success Response

```json
{
  "token": "<token>",
  "expires_in": 86400
}
```

### Failure Response

Authentication failure returns `401` and does not affect other saved profiles.

## Compatibility Rules

- The client must probe discovery before prompting for or submitting remote credentials.
- Server-side additions to discovery must be additive and optional to preserve compatibility.
- The client may only treat `secret_key` as a supported remote auth mode in this feature.
