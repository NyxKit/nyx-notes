export enum InstallationMode {
  Local = 'local',
  Server = 'server',
}

export enum ServerSetupChoice {
  SetupNewServer = 'setup_new_server',
  ConnectExistingServer = 'connect_existing_server',
}

export enum InitialSetupState {
  Uninitialized = 'uninitialized',
  Initializing = 'initializing',
  Initialized = 'initialized',
}
