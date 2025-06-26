import * as sdk from 'matrix-js-sdk';
import { MatrixConfig, Room, User, Message, MessageType, UserStatus } from '@/types';

export class MatrixClientWrapper {
  private client: sdk.MatrixClient | null = null;
  private config: MatrixConfig | null = null;
  private eventHandlers: Map<string, Function[]> = new Map();

  constructor() {
    // Initialize event handlers map
    this.eventHandlers.set('room.timeline', []);
    this.eventHandlers.set('room.membership', []);
    this.eventHandlers.set('user.presence', []);
    this.eventHandlers.set('room.knock', []);
    this.eventHandlers.set('error', []);
  }

  async login(username: string, password: string, baseUrl: string = 'http://localhost:6167'): Promise<void> {
    try {
      // Create temporary client for login
      const tempClient = sdk.createClient({ baseUrl });

      // Convert username to full Matrix ID if needed
      const fullUserId = username.startsWith('@') ? username : `@${username}:nok.local`;
      
      console.log(`Attempting login with user: ${fullUserId} to ${baseUrl}`);

      // Attempt login with identifier format
      const response = await tempClient.login('m.login.password', {
        identifier: {
          type: 'm.id.user',
          user: fullUserId,
        },
        password: password,
      });

      // Store configuration
      this.config = {
        baseUrl,
        userId: response.user_id,
        accessToken: response.access_token,
        deviceId: response.device_id,
      };

      // Create authenticated client
      this.client = sdk.createClient({
        baseUrl: this.config.baseUrl,
        accessToken: this.config.accessToken,
        userId: this.config.userId,
        deviceId: this.config.deviceId,
      });

      // Set up event listeners
      this.setupEventListeners();

      // Start client
      await this.client.startClient({ initialSyncLimit: 10 });

    } catch (error) {
      throw new Error(`Login failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  async logout(): Promise<void> {
    if (this.client) {
      await this.client.logout();
      this.client.stopClient();
      this.client = null;
      this.config = null;
    }
  }

  private setupEventListeners(): void {
    if (!this.client) return;

    // Room timeline events (messages)
    this.client.on('Room.timeline' as any, (event: any, room: any, toStartOfTimeline: any) => {
      if (toStartOfTimeline) return; // Ignore historical events

      const handlers = this.eventHandlers.get('room.timeline') || [];
      handlers.forEach(handler => {
        try {
          handler(this.convertMatrixEventToMessage(event, room));
        } catch (error) {
          console.error('Error in room.timeline handler:', error);
        }
      });
    });

    // Room membership events
    this.client.on('RoomMember.membership' as any, (event: any, member: any) => {
      const handlers = this.eventHandlers.get('room.membership') || [];
      handlers.forEach(handler => {
        try {
          handler(member);
        } catch (error) {
          console.error('Error in room.membership handler:', error);
        }
      });
    });

    // User presence events
    this.client.on('User.presence' as any, (event: any, user: any) => {
      const handlers = this.eventHandlers.get('user.presence') || [];
      handlers.forEach(handler => {
        try {
          handler(this.convertMatrixUserToUser(user));
        } catch (error) {
          console.error('Error in user.presence handler:', error);
        }
      });
    });

    // Error events
    this.client.on('sync' as any, (state: any, prevState: any, data: any) => {
      if (state === 'ERROR') {
        const handlers = this.eventHandlers.get('error') || [];
        handlers.forEach(handler => {
          try {
            handler(data);
          } catch (error) {
            console.error('Error in error handler:', error);
          }
        });
      }
    });
  }

  on(eventType: string, handler: Function): void {
    const handlers = this.eventHandlers.get(eventType) || [];
    handlers.push(handler);
    this.eventHandlers.set(eventType, handlers);
  }

  off(eventType: string, handler: Function): void {
    const handlers = this.eventHandlers.get(eventType) || [];
    const index = handlers.indexOf(handler);
    if (index > -1) {
      handlers.splice(index, 1);
      this.eventHandlers.set(eventType, handlers);
    }
  }

  async getRooms(): Promise<Room[]> {
    if (!this.client) return [];

    const rooms = this.client.getRooms();
    return rooms.map(room => this.convertMatrixRoomToRoom(room));
  }

  async getUsers(roomId?: string): Promise<User[]> {
    if (!this.client) return [];

    if (roomId) {
      const room = this.client.getRoom(roomId);
      if (!room) return [];

      const members = room.getJoinedMembers();
      return members.map(member => this.convertMatrixUserToUser(member.user)).filter(user => user !== null) as User[];
    }

    // Get all known users
    const users = this.client.getUsers();
    return users.map(user => this.convertMatrixUserToUser(user)).filter(user => user !== null) as User[];
  }

  async sendMessage(roomId: string, content: string): Promise<void> {
    if (!this.client) throw new Error('Not connected to Matrix');

    await this.client.sendTextMessage(roomId, content);
  }

  async sendKnock(userId: string, message?: string): Promise<void> {
    if (!this.client) throw new Error('Not connected to Matrix');

    // Find or create DM room with user
    const dmRoomId = await this.getOrCreateDMRoom(userId);

    // Send custom knock event
    await this.client.sendEvent(dmRoomId, 'com.nok.knock', {
      message: message || 'knock knock!',
      timestamp: Date.now(),
    });
  }

  async joinRoom(roomIdOrAlias: string): Promise<void> {
    if (!this.client) throw new Error('Not connected to Matrix');

    await this.client.joinRoom(roomIdOrAlias);
  }

  private async getOrCreateDMRoom(userId: string): Promise<string> {
    if (!this.client) throw new Error('Not connected to Matrix');

    // Try to find existing DM room
    const rooms = this.client.getRooms();
    for (const room of rooms) {
      if (room.getJoinedMemberCount() === 2) {
        const members = room.getJoinedMembers();
        const memberIds = members.map(m => m.userId);
        if (memberIds.includes(userId) && memberIds.includes(this.config!.userId)) {
          return room.roomId;
        }
      }
    }

    // Create new DM room
    const result = await this.client.createRoom({
      invite: [userId],
      is_direct: true,
      preset: 'trusted_private_chat' as any,
    });

    return result.room_id;
  }

  private convertMatrixRoomToRoom(matrixRoom: sdk.Room): Room {
    return {
      id: matrixRoom.roomId,
      name: matrixRoom.name || matrixRoom.getCanonicalAlias() || 'Unnamed Room',
      alias: matrixRoom.getCanonicalAlias() || undefined,
      topic: matrixRoom.currentState.getStateEvents('m.room.topic', '')?.getContent()?.topic,
      members: matrixRoom.getJoinedMembers().map(m => m.userId),
      avatarUrl: matrixRoom.getAvatarUrl(this.client!.baseUrl, 64, 64, 'crop') || undefined,
    };
  }

  private convertMatrixUserToUser(matrixUser: any): User | null {
    if (!matrixUser) return null;
    
    const presence = matrixUser.presence || 'offline';
    let status: UserStatus = UserStatus.Offline;

    switch (presence) {
      case 'online':
        status = UserStatus.Online;
        break;
      case 'unavailable':
        status = UserStatus.Away;
        break;
      case 'busy':
        status = UserStatus.Busy;
        break;
      default:
        status = UserStatus.Offline;
    }

    return {
      id: matrixUser.userId,
      name: matrixUser.displayName || matrixUser.userId.split(':')[0].substring(1),
      status,
      avatar: matrixUser.avatarUrl,
    };
  }

  private convertMatrixEventToMessage(event: sdk.MatrixEvent, room: sdk.Room): Message {
    const content = event.getContent();
    const eventType = event.getType();

    let messageType: MessageType = MessageType.Text;
    let messageContent = content.body || '';

    switch (eventType) {
      case 'm.room.message':
        switch (content.msgtype) {
          case 'm.text':
            messageType = MessageType.Text;
            break;
          case 'm.notice':
            messageType = MessageType.Notice;
            break;
          case 'm.emote':
            messageType = MessageType.Emote;
            break;
        }
        break;
      case 'com.nok.knock':
        messageType = MessageType.Knock;
        messageContent = content.message || 'knock knock!';
        break;
    }

    return {
      id: event.getId() || '',
      sender: event.getSender() || '',
      content: messageContent,
      timestamp: new Date(event.getTs()),
      roomId: room.roomId,
      type: messageType,
    };
  }

  isConnected(): boolean {
    return this.client !== null && this.client.getSyncState() === 'SYNCING';
  }

  getCurrentUser(): User | null {
    if (!this.client || !this.config) return null;

    const matrixUser = this.client.getUser(this.config.userId);
    if (!matrixUser) return null;
    
    const user = this.convertMatrixUserToUser(matrixUser);
    return user;
  }
}

// Singleton instance
export const matrixClient = new MatrixClientWrapper();