export enum RouteName {
  Home = 'home',
  ServerRoot = 'server-root',
  ServerVault = 'server-vault',
  ServerNote = 'server-note',
  ServerVaultSettings = 'server-vault-settings',
  UserRoot = 'user-root',
  UserVault = 'user-vault',
  UserNote = 'user-note',
  UserVaultSettings = 'user-vault-settings',
  Search = 'search',
  Favorites = 'favorites',
  Feedback = 'feedback',
  FeedbackNote = 'feedback-note',
  Users = 'users',
  Settings = 'settings',
  Login = 'login',
  Setup = 'setup',
}

export enum RoutePath {
  Home = '/',
  Search = '/search',
  Favorites = '/favorites',
  Settings = '/settings',
  Users = '/users',
  Login = '/login',
  Setup = '/setup',
}

export enum RouteQueryKey {
  Add = 'add',
  Manage = 'manage',
  Profile = 'profile',
  Redirect = 'redirect',
}

export enum RouteQueryValue {
  Remote = 'remote',
  Active = 'active',
}
