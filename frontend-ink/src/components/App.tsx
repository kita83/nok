import React, { useEffect } from 'react';
import { Box, useInput, useApp } from 'ink';
import { useAppStore } from '@/store/appStore';
import { useMatrix } from '@/hooks/useMatrix';
import { ViewMode, PaneType } from '@/types';
import LoginScreen from './LoginScreen';
import MainScreen from './MainScreen';
import SettingsScreen from './SettingsScreen';

const App: React.FC = () => {
  const { exit } = useApp();
  const {
    currentView,
    isInputMode,
    focusedPane,
    cycleFocus,
    navigateUp,
    navigateDown,
    setCurrentView,
    setInputMode,
    setInputValue,
    inputValue,
    users,
    rooms,
    selectedUserIndex,
    selectedRoomIndex,
    setCurrentRoom,
    setNotification,
  } = useAppStore();

  const { sendMessage, sendKnock, joinRoom } = useMatrix();

  useInput((input, key) => {
    // Global quit command
    if (input === 'q' && !isInputMode && currentView !== ViewMode.Login) {
      exit();
      return;
    }

    // Handle different views
    if (currentView === ViewMode.Login) {
      // Login screen handles its own input
      return;
    }

    if (currentView === ViewMode.Settings) {
      if (key.escape) {
        setCurrentView(ViewMode.Main);
      }
      return;
    }

    // Main screen input handling
    if (isInputMode) {
      if (key.escape) {
        setInputMode(false);
        setInputValue('');
      } else if (key.return) {
        // Handle message sending
        if (inputValue.trim()) {
          sendMessage(inputValue.trim());
        }
        setInputMode(false);
        setInputValue('');
      } else if (key.backspace) {
        setInputValue(inputValue.slice(0, -1));
      } else if (input && !key.ctrl && !key.meta) {
        setInputValue(inputValue + input);
      }
    } else {
      // Navigation mode
      if (key.tab) {
        cycleFocus();
      } else if (key.upArrow || input === 'k') {
        navigateUp();
      } else if (key.downArrow || input === 'j') {
        navigateDown();
      } else if (input === 'i') {
        setInputMode(true);
      } else if (input === 's') {
        setCurrentView(ViewMode.Settings);
      } else if (key.return) {
        // Handle Enter key based on focused pane
        if (focusedPane === PaneType.Rooms && rooms.length > 0) {
          const selectedRoom = rooms[selectedRoomIndex];
          if (selectedRoom) {
            setCurrentRoom(selectedRoom);
            joinRoom(selectedRoom.id);
          }
        }
      } else if (input === 'k' && focusedPane === PaneType.Users) {
        // Send knock to selected user
        if (users.length > 0) {
          const selectedUser = users[selectedUserIndex];
          if (selectedUser) {
            sendKnock(selectedUser.id);
          }
        }
      }
    }
  });

  // Initialize the app
  useEffect(() => {
    // Set up initial state, load configuration, etc.
    // Add some sample data for testing
    if (currentView === ViewMode.Main && rooms.length === 0) {
      // This would normally be loaded from Matrix
      setNotification('Loading rooms and users...');
    }
  }, [currentView]);

  return (
    <Box flexDirection="column" height="100%">
      {currentView === ViewMode.Login && <LoginScreen />}
      {currentView === ViewMode.Main && <MainScreen />}
      {currentView === ViewMode.Settings && <SettingsScreen />}
    </Box>
  );
};

export default App;