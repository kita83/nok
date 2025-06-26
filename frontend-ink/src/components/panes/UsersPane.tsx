import React from 'react';
import { Box, Text } from 'ink';
import { useAppStore } from '@/store/appStore';
import { PaneType, UserStatus } from '@/types';

const UsersPane: React.FC = () => {
  const {
    users,
    selectedUserIndex,
    focusedPane,
  } = useAppStore();

  const isFocused = focusedPane === PaneType.Users;
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

  const getStatusColor = (status: UserStatus): string => {
    switch (status) {
      case UserStatus.Online:
        return 'green';
      case UserStatus.Away:
        return 'yellow';
      case UserStatus.Busy:
        return 'red';
      case UserStatus.Offline:
        return 'gray';
      default:
        return 'gray';
    }
  };

  return (
    <Box
      flexDirection="column"
      borderStyle="round"
      borderColor={borderColor}
      padding={1}
      height="100%"
    >
      <Text bold color={isFocused ? 'cyan' : 'white'}>
        Users
      </Text>
      
      <Box flexDirection="column" flexGrow={1}>
        {users.length === 0 ? (
          <Text color="gray">No users online</Text>
        ) : (
          users.map((user, index) => {
            const isSelected = isFocused && index === selectedUserIndex;
            const statusIcon = getStatusIcon(user.status);
            const statusColor = getStatusColor(user.status);
            
            const prefix = isSelected ? '> ' : '  ';
            const backgroundColor = isSelected ? 'black' : undefined;
            
            return (
              <Text
                key={user.id}
                color={isSelected ? 'cyan' : 'white'}
                backgroundColor={backgroundColor}
              >
                {prefix}
                <Text color={statusColor}>
                  {statusIcon}
                </Text>
                {' '}{user.name}
              </Text>
            );
          })
        )}
      </Box>
    </Box>
  );
};

export default UsersPane;