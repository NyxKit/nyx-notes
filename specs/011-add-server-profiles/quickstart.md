# Quickstart: Multi-Instance Access Profiles

## Goal

Verify that one client can initialize in local mode, connect to multiple remote servers, and switch profiles without leaking auth or content between them.

## Prerequisites

- A local Nyx Notes instance available for local-mode setup
- At least one reachable self-hosted server using username/password login
- Optional: a second account on the same remote server to verify same-URL multi-account support

## Scenario 1: First-Time Local Setup

1. Start from a fresh client install with no saved profiles.
2. Choose `Local` during onboarding.
3. Confirm the app opens without a login screen.
4. Confirm a local workspace profile appears in the profile switcher.

Expected result: The local profile is active and usable without authentication.

## Scenario 2: Connect To An Existing Remote Server

1. Open profile management.
2. Choose `Add profile` then `Connect to an existing server`.
3. Enter the remote server URL, username, and password.
4. Complete discovery and sign-in.

Expected result: A new remote profile is saved and becomes selectable without altering the local profile.

## Scenario 3: Same Server, Different Username

1. Add another remote profile using the same server URL.
2. Use a different username and valid password.
3. Save the profile.

Expected result: The client stores both profiles separately and shows both in the switcher.

## Scenario 4: Profile Switching Isolation

1. Switch from the local profile to remote profile A.
2. Confirm only profile A's account context and content appear.
3. Switch to remote profile B.
4. Confirm profile A's data disappears before profile B data loads.
5. Switch back to local.

Expected result: No stale notes, vaults, comments, or auth state from the previous profile remain visible after each switch.

## Scenario 5: Remote Failure Isolation

1. Edit the saved password for one remote profile so it becomes invalid.
2. Attempt to activate that profile.
3. Observe the error.
4. Switch to another valid profile.

Expected result: The invalid profile shows an isolated auth failure while the other profiles remain available and functional.

## Validation Focus

- Local mode never prompts for login
- Remote mode uses username/password only for this feature
- Same URL with different usernames is supported
- Duplicate same URL plus same username is rejected
- Removing or breaking one profile does not affect any other profile
