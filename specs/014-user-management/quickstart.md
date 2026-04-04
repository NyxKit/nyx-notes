# Quickstart: User Management

## Goal

Verify that an administrator can manage server users from the application UI while all credential handling remains inside the backend.

## Prerequisites

- A Nyx Notes server running with an auth mode that supports managed users
- An administrator account able to sign in to the application shell
- A clean or known test environment where creating and deleting test users is safe

## Scenario 1: Open The Users Page

1. Sign in as an administrator.
2. Open the authenticated application shell.
3. Inspect the lower sidebar navigation.
4. Click `Users`.

Expected result: `Users` appears directly above `Settings`, and the users page loads a tabular list of existing accounts.

## Scenario 2: Create A User

1. Click `Add user` from the users page header.
2. Enter a unique username, display name, role, and valid password.
3. Save the form.

Expected result: The modal closes, the new account appears in the table, and the account can be used for future authentication.

## Scenario 3: Edit A User

1. Select `Edit` for an existing non-protected account.
2. Change the display name and optionally rotate the password.
3. Save the form.

Expected result: The table reflects the updated metadata, and the prior password no longer works if a new one was supplied.

## Scenario 4: Reject Duplicate Usernames

1. Start creating a new user.
2. Enter a username already used by another account.
3. Submit the form.

Expected result: The create action fails with a clear validation message, and no duplicate account is created.

## Scenario 5: Delete A User

1. Select `Delete` for an eligible non-protected account.
2. Confirm the deletion.
3. Refresh the users page or reopen it.

Expected result: The account is gone from the list and cannot sign in anymore.

## Scenario 6: Block Protected Admin Removal

1. Identify the last remaining administrator account in the environment.
2. Attempt to delete it or change it to a non-admin role.

Expected result: The backend rejects the action and the UI explains that the server must retain administrative access.

## Validation Focus

- Sidebar placement matches the feature specification
- Users list shows current backend state, not cached stale data
- All create/edit/delete actions are administrator-only
- Passwords are never shown after save
- Protected-account rules prevent accidental lockout
