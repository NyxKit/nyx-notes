# Feature Specification: Multi-Instance Access Profiles

**Feature Branch**: `011-add-server-profiles`  
**Created**: 2026-03-30  
**Status**: Draft  
**Input**: User description: "this project should allow both hosting on a nas (setup through docker) and a local instance. users should be able to connect to as many server instances as they like and the client should allow switching between \"local\", \"server 1\", \"server 2\", etc. Each server however maintains its own authentication and user management. So connecting to a new server should have: url, username, password all tied specifically to that server, setup by the server manager when setting up our app, the user should initialize the project as \"local\" (= no auth required) or \"server\" (two options: setup a new server, connect to existing server)"

## Clarifications

### Session 2026-03-30

- Q: Which remote authentication modes must this feature support? → A: Username/password only for remote profiles in this feature.
- Q: Can the same server URL be saved more than once? → A: Yes, if each saved profile uses a different username.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Choose an Initial Operating Mode (Priority: P1)

As a person setting up the app for the first time, I want to choose whether I am using it as a local-only instance or as a server-connected experience so the app starts in a mode that matches how I plan to use it.

**Why this priority**: The product cannot be used correctly until setup establishes whether authentication is required and whether the app should behave as a standalone local workspace or as a client for a remote server.

**Independent Test**: Can be fully tested by starting from a fresh install, completing setup in local mode and server mode separately, and confirming the app enters the correct post-setup state in each case.

**Acceptance Scenarios**:

1. **Given** a first-time user with no existing setup, **When** they choose `Local` during onboarding, **Then** the app creates a local workspace profile, skips any login requirement, and opens the app in local mode.
2. **Given** a first-time user with no existing setup, **When** they choose `Server` during onboarding, **Then** the app offers both `Set up a new server` and `Connect to an existing server` as distinct next steps.
3. **Given** a first-time user chooses `Connect to an existing server`, **When** they provide a valid server URL, username, and password, **Then** the app saves that server connection as a selectable profile and signs the user into that server profile.

---

### User Story 2 - Manage Multiple Server Connections (Priority: P2)

As a user who works with more than one hosted Nyx Notes server, I want to add and keep multiple server profiles in the same client so I can switch between personal, local, and self-hosted environments without re-entering connection details every time.

**Why this priority**: Multi-instance access is the core value of the requested feature and enables the client to act as a hub for local usage plus several independent server deployments.

**Independent Test**: Can be fully tested by adding at least two server profiles plus one local profile, switching among them, and confirming the selected profile becomes the active workspace without mixing data from other profiles.

**Acceptance Scenarios**:

1. **Given** a user already has one active profile, **When** they add another server connection with a valid URL and valid credentials, **Then** the app stores it as a separate selectable profile without overwriting the existing one.
2. **Given** a user already saved one account for a server URL, **When** they add the same server URL again with a different username, **Then** the app saves it as a separate selectable profile.
3. **Given** a user has `Local`, `Server 1`, and `Server 2` profiles available, **When** they switch from one profile to another, **Then** the app updates the active workspace to the selected profile and shows only the content and account context for that profile.
4. **Given** a user has multiple saved server profiles, **When** they view the profile switcher, **Then** they can clearly distinguish local and remote profiles by name and server address.

---

### User Story 3 - Keep Authentication Bound to Each Server (Priority: P3)

As a user connecting to multiple servers managed by different administrators, I want each server to keep its own username, password, and signed-in state so one server's account never affects access to another server.

**Why this priority**: The user explicitly requires server-specific authentication and user management. Preserving that separation avoids accidental cross-server access and confusion.

**Independent Test**: Can be fully tested by saving two different server profiles with different credentials, signing into each one, and verifying that a failure or sign-out on one server does not change the other profile.

**Acceptance Scenarios**:

1. **Given** two saved server profiles, **When** the user signs out of one server profile, **Then** the other server profile remains saved and retains its own authentication state.
2. **Given** a saved server profile with outdated credentials, **When** the server rejects authentication, **Then** the app shows the failure only for that profile and does not block access to other saved profiles.
3. **Given** a server profile is selected, **When** the user updates that profile's username or password, **Then** the app applies the change only to that profile.

---

### Edge Cases

- What happens when a user enters a server URL that is unreachable or invalid during setup or while adding a new profile?
- What happens when two saved server profiles point to the same server URL but use different usernames? The system must allow both profiles to coexist as separate account contexts.
- How does the system handle a server profile whose credentials were valid when saved but later become invalid because the server administrator changed them?
- How does the system behave if the user removes the currently active profile?
- What happens when a previously available server changes from requiring authentication to local-only access, or the reverse?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST support two installation modes during first-time setup: `Local` and `Server`.
- **FR-002**: When `Local` is selected, the system MUST create a local workspace profile that can be used without authentication.
- **FR-003**: When `Server` is selected, the system MUST offer both `Set up a new server` and `Connect to an existing server` as onboarding choices.
- **FR-004**: The system MUST allow users to save one or more server connection profiles in a single client installation.
- **FR-005**: Each server connection profile MUST include a server URL, username, and password provided for that specific server.
- **FR-006**: The system MUST treat each saved server connection profile as an independent account context with its own authentication state.
- **FR-007**: The system MUST allow users to switch the active workspace between any saved local or server profile.
- **FR-008**: When a profile is active, the system MUST show only the notes, vaults, and account context associated with that profile.
- **FR-009**: The system MUST preserve saved profiles across app restarts until the user explicitly removes them.
- **FR-010**: The system MUST let users review and edit the connection details for an existing server profile.
- **FR-011**: The system MUST let users remove a saved server profile without affecting any other saved profile.
- **FR-012**: Authentication failures for one server profile MUST be reported within that profile's context and MUST NOT sign the user out of, disable, or corrupt other saved profiles.
- **FR-013**: The system MUST clearly distinguish local profiles from remote server profiles in the profile switcher.
- **FR-014**: The system MUST let a user create and keep more than one remote profile even when those profiles connect to different servers managed by different administrators.
- **FR-015**: The system MUST support server deployments that are intended for self-hosted environments such as NAS devices, alongside local-only usage on a single machine.
- **FR-016**: The system MUST require successful username/password authentication before granting access to a remote server profile.
- **FR-017**: The system MUST keep server-managed user identities and permissions scoped to the server that issued them.
- **FR-018**: For this feature, remote server connections MUST use a username-and-password sign-in flow and MUST NOT require support for external identity-provider login flows.
- **FR-019**: The system MUST allow multiple saved remote profiles for the same server URL when each profile uses a different username.

### Key Entities *(include if feature involves data)*

- **Workspace Profile**: A selectable app context representing either a local workspace or a connection to a remote server. Key attributes include profile type, display name, active status, and whether authentication is required.
- **Server Connection Profile**: A remote workspace profile that stores the server address and the user credentials associated with that server, plus the profile's connection and sign-in status.
- **Server Account Session**: The current authenticated access state for one specific server profile, including whether sign-in succeeded, failed, or requires renewal. It is never shared across profiles.
- **Installation Mode**: The initial setup choice that determines whether the app starts as a local-only experience or a server-oriented experience.

## Assumptions

- A single client installation may contain both one local profile and multiple remote server profiles.
- Users can assign recognizable labels such as `Local`, `Server 1`, or `Work NAS` to saved profiles so they are easy to distinguish.
- `Set up a new server` is part of the onboarding flow for server-oriented users, but the detailed server provisioning steps are outside the scope of this feature.
- The server administrator, not the client user, controls user creation, password resets, and permission assignment for each remote server.
- Existing local-only behavior remains available and is not removed by the introduction of remote profiles.
- Remote profiles in this feature use only username/password authentication; support for other remote authentication methods can be considered separately later.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In usability testing, at least 90% of first-time users can complete setup into either local mode or a connected server profile without external help.
- **SC-002**: A user can add a new remote server profile and switch to it in under 2 minutes when the correct server URL and credentials are available.
- **SC-003**: In acceptance testing with one local profile and at least three remote profiles, 100% of profile switches show the correct profile-specific workspace without exposing content from another profile.
- **SC-004**: In authentication failure tests, 100% of failed sign-in events remain isolated to the affected server profile and do not interrupt access to other valid saved profiles.
- **SC-005**: At least 95% of users in validation testing can correctly identify which profile is local and which profiles are remote from the profile switcher alone.
