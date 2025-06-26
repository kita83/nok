import { create } from 'zustand';
import {
  AppState,
  ViewMode,
  PaneType,
  ConnectionStatus,
  User,
  Room,
  Message,
  UserStatus,
} from '@/types';

interface AppStore extends AppState {
  // State
  users: User[];
  rooms: Room[];
  messages: Message[];
  selectedRoomIndex: number;
  selectedUserIndex: number;
  selectedMessageIndex: number;
  inputValue: string;
  error: string | null;
  notification: string | null;
  
  // Actions
  setCurrentView: (view: ViewMode) => void;
  setCurrentUser: (user: User | null) => void;
  setCurrentRoom: (room: Room | null) => void;
  setFocusedPane: (pane: PaneType) => void;
  setInputMode: (enabled: boolean) => void;
  setConnectionStatus: (status: ConnectionStatus) => void;
  
  // Data actions
  setUsers: (users: User[]) => void;
  setRooms: (rooms: Room[]) => void;
  setMessages: (messages: Message[]) => void;
  addMessage: (message: Message) => void;
  
  // UI actions
  setSelectedRoomIndex: (index: number) => void;
  setSelectedUserIndex: (index: number) => void;
  setSelectedMessageIndex: (index: number) => void;
  setInputValue: (value: string) => void;
  setError: (error: string | null) => void;
  setNotification: (notification: string | null) => void;
  
  // Utility actions
  cycleFocus: () => void;
  navigateUp: () => void;
  navigateDown: () => void;
  resetSelections: () => void;
}

export const useAppStore = create<AppStore>((set, get) => ({
  // Initial state
  currentView: ViewMode.Login,
  currentUser: null,
  currentRoom: null,
  focusedPane: PaneType.Rooms,
  isInputMode: false,
  connectionStatus: ConnectionStatus.Disconnected,
  
  users: [],
  rooms: [],
  messages: [],
  selectedRoomIndex: 0,
  selectedUserIndex: 0,
  selectedMessageIndex: 0,
  inputValue: '',
  error: null,
  notification: null,
  
  // Actions
  setCurrentView: (view) => set({ currentView: view }),
  setCurrentUser: (user) => set({ currentUser: user }),
  setCurrentRoom: (room) => set({ currentRoom: room }),
  setFocusedPane: (pane) => set({ focusedPane: pane }),
  setInputMode: (enabled) => set({ isInputMode: enabled }),
  setConnectionStatus: (status) => set({ connectionStatus: status }),
  
  // Data actions
  setUsers: (users) => set({ users }),
  setRooms: (rooms) => set({ rooms }),
  setMessages: (messages) => set({ messages }),
  addMessage: (message) => set((state) => ({ 
    messages: [...state.messages, message] 
  })),
  
  // UI actions
  setSelectedRoomIndex: (index) => set({ selectedRoomIndex: index }),
  setSelectedUserIndex: (index) => set({ selectedUserIndex: index }),
  setSelectedMessageIndex: (index) => set({ selectedMessageIndex: index }),
  setInputValue: (value) => set({ inputValue: value }),
  setError: (error) => set({ error }),
  setNotification: (notification) => set({ notification }),
  
  // Utility actions
  cycleFocus: () => {
    const panes = [PaneType.Rooms, PaneType.Users, PaneType.Messages, PaneType.Status];
    const current = get().focusedPane;
    const currentIndex = panes.indexOf(current);
    const nextIndex = (currentIndex + 1) % panes.length;
    set({ focusedPane: panes[nextIndex] });
  },
  
  navigateUp: () => {
    const state = get();
    switch (state.focusedPane) {
      case PaneType.Rooms:
        if (state.selectedRoomIndex > 0) {
          set({ selectedRoomIndex: state.selectedRoomIndex - 1 });
        }
        break;
      case PaneType.Users:
        if (state.selectedUserIndex > 0) {
          set({ selectedUserIndex: state.selectedUserIndex - 1 });
        }
        break;
      case PaneType.Messages:
        if (state.selectedMessageIndex > 0) {
          set({ selectedMessageIndex: state.selectedMessageIndex - 1 });
        }
        break;
    }
  },
  
  navigateDown: () => {
    const state = get();
    switch (state.focusedPane) {
      case PaneType.Rooms:
        if (state.selectedRoomIndex < state.rooms.length - 1) {
          set({ selectedRoomIndex: state.selectedRoomIndex + 1 });
        }
        break;
      case PaneType.Users:
        if (state.selectedUserIndex < state.users.length - 1) {
          set({ selectedUserIndex: state.selectedUserIndex + 1 });
        }
        break;
      case PaneType.Messages:
        if (state.selectedMessageIndex < state.messages.length - 1) {
          set({ selectedMessageIndex: state.selectedMessageIndex + 1 });
        }
        break;
    }
  },
  
  resetSelections: () => set({
    selectedRoomIndex: 0,
    selectedUserIndex: 0,
    selectedMessageIndex: 0,
  }),
}));