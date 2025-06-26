export interface User {
  id: string;
  name: string;
  status: UserStatus;
  avatar?: string;
}

export enum UserStatus {
  Online = 'online',
  Away = 'away',
  Busy = 'busy',
  Offline = 'offline',
}

export interface Room {
  id: string;
  name: string;
  alias?: string;
  topic?: string;
  members: string[];
  avatarUrl?: string;
}

export interface Message {
  id: string;
  sender: string;
  content: string;
  timestamp: Date;
  roomId: string;
  type: MessageType;
}

export enum MessageType {
  Text = 'm.text',
  Knock = 'com.nok.knock',
  Notice = 'm.notice',
  Emote = 'm.emote',
}

export interface AppState {
  currentView: ViewMode;
  currentUser: User | null;
  currentRoom: Room | null;
  focusedPane: PaneType;
  isInputMode: boolean;
  connectionStatus: ConnectionStatus;
}

export enum ViewMode {
  Login = 'login',
  Main = 'main',
  Settings = 'settings',
}

export enum PaneType {
  Rooms = 'rooms',
  Users = 'users',
  Messages = 'messages',
  Status = 'status',
}

export enum ConnectionStatus {
  Disconnected = 'disconnected',
  Connecting = 'connecting',
  Connected = 'connected',
  Error = 'error',
}

export interface MatrixConfig {
  baseUrl: string;
  userId: string;
  accessToken?: string;
  deviceId?: string;
}

export interface AppConfig {
  matrix: MatrixConfig;
  audio: {
    enabled: boolean;
    knockSoundPath?: string;
  };
  ui: {
    theme: 'dark' | 'light';
    shortcuts: Record<string, string>;
  };
}

export interface KnockEvent {
  sender: string;
  target: string;
  timestamp: Date;
  message?: string;
}