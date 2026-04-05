# Feature Specification: User Management

**Feature Branch**: `014-user-management`  
**Created**: 2026-04-03  
**Status**: Draft  
**Input**: User description: "i want to introduce user management, this means the following:
- we need a fast, small and local database to store username + password (encrypted) - perhaps sqlite, maybe others? propose something
- everything auth and user related is funneled through the rust backend. only the rust backend talks to the database, the frontend talks to the rust backend through the api
- a new sidebar nav entry (bottom, just above \"settings\") called \"users\" needs to be added
- a new page to manage users
  - use NyxTable for a user list
  - use a column for the following actions: edit, delete (nyxbutton, nyxicon)
- use NyxButton for \"add user\" in the header actions
- new component \"CreateEditUser\" => NyxModel underneath
- use the top-level (codebase src) domain \"users\" for everything related to users"

## Clarifications

### Session 2026-04-04

- Q: Which authentication modes does MVP user management apply to? → A: `secret_key` only for MVP; `oidc` may be supported later, and `local` is out of scope.
- Q: What minimum password policy should MVP enforce? → A: Passwords must be at least 12 characters and include at least 3 of these 4 categories: lowercase letters, uppercase letters, digits, and symbols.
- Q: Is email required for each managed user in MVP? → A: Yes, every managed user must have an email address.
- Q: How should self-management work for the currently signed-in administrator? → A: Administrators may edit themselves, but may not delete themselves or demote themselves from admin.
- Q: Must managed user email addresses be unique? → A: Yes, email addresses are required and unique across all managed users.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Review Existing Users (Priority: P1)

As an administrator, I can open a dedicated users management page from the main sidebar and review all existing user accounts in one place so I can understand who currently has access to the server.

**Why this priority**: Administrators need visibility into existing accounts before they can safely manage access.

**Independent Test**: Can be fully tested by signing in as an administrator, opening the users page, and confirming that the current set of user accounts is listed with their core account details and available actions.

**Acceptance Scenarios**:

1. **Given** an authenticated administrator with permission to manage users, **When** they open the application sidebar, **Then** they see a "Users" navigation entry positioned directly above "Settings".
2. **Given** an authenticated administrator, **When** they open the users page, **Then** they see a tabular list of user accounts with per-user actions for editing and deleting accounts.
3. **Given** the system contains no managed users beyond any required bootstrap account, **When** an administrator opens the users page, **Then** the page clearly communicates the empty or near-empty state and still offers a visible action to add a user.

---

### User Story 2 - Create And Update Users (Priority: P1)

As an administrator, I can add a new user or edit an existing user from the users page so I can keep account details and credentials up to date without direct access to server storage.

**Why this priority**: The feature delivers its core value only if administrators can create and maintain accounts through the product itself.

**Independent Test**: Can be fully tested by creating a user from the users page, verifying the new account appears in the list, editing that user, and confirming the updated details are reflected correctly.

**Acceptance Scenarios**:

1. **Given** an authenticated administrator on the users page, **When** they choose to add a user, **Then** the system opens a dedicated create-user form and allows them to submit required account details.
2. **Given** an authenticated administrator viewing an existing user, **When** they choose to edit that user, **Then** the system opens a dedicated edit-user form populated with the current account details.
3. **Given** an administrator submits invalid or incomplete user details, **When** the system validates the request, **Then** it blocks the change and explains what must be corrected.
4. **Given** an administrator successfully creates or updates a user, **When** the operation completes, **Then** the user list reflects the saved change without requiring direct access to storage.

---

### User Story 3 - Remove Users Safely (Priority: P2)

As an administrator, I can delete a user account when that person should no longer have access so the server's active user list stays accurate and controlled.

**Why this priority**: Removing access is critical for administration, but it depends on the user list and management page already existing.

**Independent Test**: Can be fully tested by deleting an eligible user from the users page and confirming the account no longer appears in the list and can no longer be used for authentication.

**Acceptance Scenarios**:

1. **Given** an authenticated administrator viewing the users list, **When** they choose to delete an eligible user, **Then** the system asks for confirmation before removing the account.
2. **Given** an administrator confirms deletion, **When** the operation succeeds, **Then** the deleted account is removed from the list and is no longer accepted for future sign-in.
3. **Given** an administrator attempts to delete a protected account, **When** the system evaluates the request, **Then** it blocks the deletion and explains why the account cannot be removed.

---

### Edge Cases

- What happens when an administrator tries to create a user with a username that already exists?
- How does the system handle a password that is shorter than 12 characters or does not include at least 3 of these 4 categories: lowercase letters, uppercase letters, digits, and symbols?
- What happens when the currently signed-in administrator edits their own account details, attempts to delete themselves, or attempts to remove their own administrator role?
- What happens when an administrator attempts to delete the last remaining administrator account?
- How does the system behave if user storage is temporarily unavailable while the users page is loading or saving changes?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST provide a dedicated user-management capability for server administrators.
- **FR-001a**: MVP user management MUST apply only to the server-managed username/password authentication mode and MUST NOT extend to single-user local mode.
- **FR-002**: The system MUST store managed user accounts in a fast, local embedded credential store controlled by the server.
- **FR-003**: The system MUST ensure that password credentials are stored securely and are never retrievable in plaintext form.
- **FR-004**: The system MUST route all user and authentication data access through the backend; client applications MUST NOT access credential storage directly.
- **FR-005**: The system MUST expose backend-managed interfaces for listing, creating, updating, and deleting user accounts.
- **FR-006**: The application MUST include a "Users" navigation entry in the sidebar directly above "Settings".
- **FR-007**: The system MUST provide a users management page that shows user accounts in a tabular list.
- **FR-008**: The users management page MUST provide row-level actions to edit and delete each eligible user account.
- **FR-009**: The users management page MUST provide a clear primary action for adding a new user.
- **FR-010**: The system MUST provide a dedicated create/edit user flow that supports both creating a new account and updating an existing account.
- **FR-011**: The system MUST validate required user details before saving changes and MUST return clear error feedback when validation fails.
- **FR-011a**: The system MUST reject passwords shorter than 12 characters and MUST reject passwords that do not include at least 3 of these 4 categories: lowercase letters, uppercase letters, digits, and symbols.
- **FR-011b**: The system MUST require an email address for every managed user and MUST reject create or update requests that omit it.
- **FR-012**: The system MUST prevent duplicate usernames.
- **FR-012a**: The system MUST prevent duplicate email addresses across all managed users.
- **FR-013**: The system MUST restrict user-management actions to authorized administrators.
- **FR-013a**: The system MUST allow an authenticated administrator to edit their own account details, but MUST block self-deletion and MUST block self-demotion from the administrator role.
- **FR-014**: The system MUST prevent deletion of protected accounts when that deletion would leave the server without required administrative access.
- **FR-015**: The system MUST make newly created or updated user accounts available for future authentication once the change is confirmed.
- **FR-016**: The system MUST remove deleted user accounts from future authentication once deletion is confirmed.

### Key Entities *(include if feature involves data)*

- **User Account**: A managed server identity with a unique username, required email address, display information, access role, lifecycle state, and audit-relevant timestamps.
- **User Credential**: The secure authentication material associated with a user account, stored in non-reversible form and managed only through backend-controlled operations.
- **User Management Session**: An authenticated administrator interaction in which user accounts are listed, created, updated, or deleted through authorized management flows.

## Assumptions

- User management is intended for server administrators, not for regular users.
- MVP user management applies only when the server is running in the server-managed username/password mode; future support for external identity-provider modes is out of scope for this feature.
- The existing bootstrap or local administration path remains available so the server can always retain at least one administrator.
- The user-management page is part of the authenticated application shell and is not available to unauthenticated visitors.
- User creation and editing cover account details and password assignment, but not broader profile, team, or permission-model redesign beyond account role/access needs for this feature.
- Every managed user record includes a required email address in addition to username, display name, role, and password.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of authenticated administrators can navigate from the main application shell to the users page in 2 clicks or fewer.
- **SC-002**: In usability testing, administrators can create a new user account and return to the user list in under 2 minutes without direct server access.
- **SC-003**: In validation testing, 100% of duplicate-username attempts are rejected before a conflicting account is created.
- **SC-004**: In administrative testing, 100% of deleted user accounts are prevented from signing in after deletion is confirmed.
- **SC-005**: In error-handling testing, 100% of failed create, update, or delete attempts return a user-visible explanation and leave existing accounts unchanged.
