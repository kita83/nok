import React from 'react';
import { Box, Text } from 'ink';
import { useAppStore } from '@/store/appStore';
import { PaneType, UserStatus } from '@/types';

const StatusPane: React.FC = () => {
  const {
    currentUser,
    currentRoom,
    connectionStatus,
    focusedPane,
    error,
    notification,
  } = useAppStore();

  const isFocused = focusedPane === PaneType.Status;
  const borderColor = isFocused ? 'cyan' : 'white';

  const getStatusIcon = (status: UserStatus): string => {
    switch (status) {
      case UserStatus.Online:
        return '●';
      case UserStatus.Away:
        return '○';
      case UserStatus.Busy:
        return '◆';
      case UserStatus.Offline:
        return '◇';
      default:
        return '◇';
    }
  };

  const getTerminalInfo = () => {
    const terminalSize = process.stdout.columns && process.stdout.rows 
      ? `${process.stdout.columns}x${process.stdout.rows}`
      : 'unknown';
    
    return {
      size: terminalSize,
      term: process.env.TERM || 'unknown',
      termProgram: process.env.TERM_PROGRAM || 'unknown',
      colorTerm: process.env.COLORTERM || 'unknown',
    };
  };

  const termInfo = getTerminalInfo();

  return (
    <Box
      flexDirection="column"
      borderStyle="round"
      borderColor={borderColor}
      padding={1}
      height="100%"
    >
      <Text bold color={isFocused ? 'cyan' : 'white'}>
        Room Visualizer / Status
      </Text>
      
      <Box flexDirection="column" flexGrow={1}>
        {/* Terminal Info */}
        <Text color="green">Terminal: {termInfo.size}</Text>
        <Text color="green">Connection: {connectionStatus}</Text>
        <Text color="green">Focus: {focusedPane}</Text>
        
        <Text> </Text>
        
        {/* Terminal Environment Details */}
        <Text color="green">Terminal Info:</Text>
        <Text color="green">TERM: {termInfo.term}</Text>
        <Text color="green">TERM_PROGRAM: {termInfo.termProgram}</Text>
        <Text color="green">COLORTERM: {termInfo.colorTerm}</Text>
        
        <Text> </Text>
        
        {/* Current Room Info */}
        {currentRoom && (
          <>
            <Text color="green">Current Room: {currentRoom.name}</Text>
            <Text> </Text>
          </>
        )}
        
        {/* Current User Status */}
        {currentUser && (
          <>
            <Text color="green">
              Your Status: {getStatusIcon(currentUser.status)} {currentUser.status}
            </Text>
            <Text> </Text>
          </>
        )}
        
        {/* Controls */}
        <Text color="green">Controls:</Text>
        <Text color="green">Tab: Switch focus</Text>
        <Text color="green">↑↓/k/j: Navigate</Text>
        <Text color="green">Enter: Select</Text>
        <Text color="green">k: Knock (Users)</Text>
        <Text color="green">i: Input mode</Text>
        <Text color="green">s: Settings</Text>
        <Text color="green">q: Quit</Text>
        
        <Text> </Text>
        
        {/* Notifications */}
        {notification && (
          <Text color="green">✅ {notification}</Text>
        )}
        
        {/* Errors */}
        {error && (
          <Text color="red">❌ {error}</Text>
        )}
        
        {/* ASCII Art Room Visualizer (placeholder) */}
        <Text> </Text>
        <Text color="green">Room Layout:</Text>
        <Text color="gray">+----------------+  +----------------+</Text>
        <Text color="gray">|     Room 1     |  |     Room 2     |</Text>
        <Text color="gray">|                |  |                |</Text>
        <Text color="gray">|  ・    ・      |  |  ・            |</Text>
        <Text color="gray">|                |  |                |</Text>
        <Text color="gray">|        ・      |  |      ・        |</Text>
        <Text color="gray">|                |  |                |</Text>
        <Text color="gray">+----------------+  +----------------+</Text>
      </Box>
    </Box>
  );
};

export default StatusPane;