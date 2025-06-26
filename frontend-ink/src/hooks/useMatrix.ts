import { useEffect, useCallback } from 'react';
import { useAppStore } from '@/store/appStore';
import { matrixClient } from '@/utils/matrixClient';
import { ViewMode, ConnectionStatus, MessageType } from '@/types';

export const useMatrix = () => {
  const {
    setConnectionStatus,
    setCurrentUser,
    setRooms,
    setUsers,
    addMessage,
    setError,
    setNotification,
    currentRoom,
  } = useAppStore();

  // Initialize Matrix event handlers
  useEffect(() => {
    // Handle new messages
    const handleMessage = (message: any) => {
      addMessage(message);
      
      // Show knock notification
      if (message.type === MessageType.Knock) {
        setNotification(`🔔 Knock from ${message.sender}`);
        // Clear notification after 3 seconds
        setTimeout(() => setNotification(null), 3000);
      }
    };

    // Handle membership changes
    const handleMembership = (member: any) => {
      // Refresh users list when membership changes
      refreshUsers();
    };

    // Handle presence changes
    const handlePresence = (user: any) => {
      // Refresh users list when presence changes
      refreshUsers();
    };

    // Handle errors
    const handleError = (error: any) => {
      setError(`Matrix error: ${error.message || 'Unknown error'}`);
      setConnectionStatus(ConnectionStatus.Error);
    };

    // Register event handlers
    matrixClient.on('room.timeline', handleMessage);
    matrixClient.on('room.membership', handleMembership);
    matrixClient.on('user.presence', handlePresence);
    matrixClient.on('error', handleError);

    // Cleanup
    return () => {
      matrixClient.off('room.timeline', handleMessage);
      matrixClient.off('room.membership', handleMembership);
      matrixClient.off('user.presence', handlePresence);
      matrixClient.off('error', handleError);
    };
  }, [addMessage, setError, setNotification, setConnectionStatus]);

  // Login function
  const login = useCallback(async (username: string, password: string, baseUrl?: string) => {
    try {
      setConnectionStatus(ConnectionStatus.Connecting);
      setError(null);

      await matrixClient.login(username, password, baseUrl);

      // Get current user info
      const currentUser = matrixClient.getCurrentUser();
      if (currentUser) {
        setCurrentUser(currentUser);
      }

      // Load initial data
      await refreshRooms();
      await refreshUsers();

      setConnectionStatus(ConnectionStatus.Connected);
      setNotification('Successfully connected to Matrix!');
      
      // Clear notification after 3 seconds
      setTimeout(() => setNotification(null), 3000);

      return true;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Login failed';
      setError(errorMessage);
      setConnectionStatus(ConnectionStatus.Error);
      return false;
    }
  }, [setConnectionStatus, setError, setCurrentUser, setNotification]);

  // Logout function
  const logout = useCallback(async () => {
    try {
      await matrixClient.logout();
      setCurrentUser(null);
      setRooms([]);
      setUsers([]);
      setConnectionStatus(ConnectionStatus.Disconnected);
      setNotification('Logged out successfully');
    } catch (error) {
      setError(`Logout failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }, [setCurrentUser, setRooms, setUsers, setConnectionStatus, setNotification, setError]);

  // Refresh rooms
  const refreshRooms = useCallback(async () => {
    try {
      const rooms = await matrixClient.getRooms();
      setRooms(rooms);
    } catch (error) {
      setError(`Failed to load rooms: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }, [setRooms, setError]);

  // Refresh users
  const refreshUsers = useCallback(async () => {
    try {
      const roomId = currentRoom?.id;
      const users = await matrixClient.getUsers(roomId);
      setUsers(users);
    } catch (error) {
      setError(`Failed to load users: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }, [setUsers, setError, currentRoom]);

  // Send message
  const sendMessage = useCallback(async (content: string) => {
    if (!currentRoom) {
      setError('No room selected');
      return false;
    }

    try {
      await matrixClient.sendMessage(currentRoom.id, content);
      return true;
    } catch (error) {
      setError(`Failed to send message: ${error instanceof Error ? error.message : 'Unknown error'}`);
      return false;
    }
  }, [currentRoom, setError]);

  // Send knock
  const sendKnock = useCallback(async (userId: string, message?: string) => {
    try {
      await matrixClient.sendKnock(userId, message);
      setNotification(`Knock sent to ${userId}`);
      
      // Clear notification after 2 seconds
      setTimeout(() => setNotification(null), 2000);
      
      return true;
    } catch (error) {
      setError(`Failed to send knock: ${error instanceof Error ? error.message : 'Unknown error'}`);
      return false;
    }
  }, [setError, setNotification]);

  // Join room
  const joinRoom = useCallback(async (roomIdOrAlias: string) => {
    try {
      await matrixClient.joinRoom(roomIdOrAlias);
      await refreshRooms();
      setNotification(`Joined room: ${roomIdOrAlias}`);
      
      // Clear notification after 2 seconds
      setTimeout(() => setNotification(null), 2000);
      
      return true;
    } catch (error) {
      setError(`Failed to join room: ${error instanceof Error ? error.message : 'Unknown error'}`);
      return false;
    }
  }, [refreshRooms, setNotification, setError]);

  // Check connection status
  const isConnected = useCallback(() => {
    return matrixClient.isConnected();
  }, []);

  return {
    login,
    logout,
    sendMessage,
    sendKnock,
    joinRoom,
    refreshRooms,
    refreshUsers,
    isConnected,
  };
};