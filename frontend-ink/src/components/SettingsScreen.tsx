import React from 'react';
import { Box, Text } from 'ink';
import { useAppStore } from '@/store/appStore';

const SettingsScreen: React.FC = () => {
  const { currentUser } = useAppStore();

  return (
    <Box flexDirection="column" padding={2}>
      <Box flexDirection="column" borderStyle="round" borderColor="cyan" padding={1}>
        <Text color="cyan" bold>
          Settings
        </Text>
      </Box>

      <Box flexDirection="column" borderStyle="round" borderColor="white" padding={1} marginTop={1}>
        <Text bold>User Information:</Text>
        <Text>Username: {currentUser?.name || 'Unknown'}</Text>
        <Text>Status: {currentUser?.status || 'Unknown'}</Text>
        <Text>ID: {currentUser?.id || 'Unknown'}</Text>
      </Box>

      <Box flexDirection="column" borderStyle="round" borderColor="white" padding={1} marginTop={1}>
        <Text bold>Audio Settings:</Text>
        <Text>• Knock sounds: Enabled</Text>
        <Text>• Text-to-speech: Disabled</Text>
      </Box>

      <Box flexDirection="column" borderStyle="round" borderColor="white" padding={1} marginTop={1}>
        <Text bold>Connection Settings:</Text>
        <Text>• Homeserver: nok.local:6167</Text>
        <Text>• Auto-reconnect: Enabled</Text>
      </Box>

      <Box flexDirection="column" borderStyle="round" borderColor="gray" padding={1} marginTop={1}>
        <Text color="gray">Help:</Text>
        <Text color="gray">• Esc: Return to main screen</Text>
        <Text color="gray">• Settings will be expanded in future versions</Text>
      </Box>
    </Box>
  );
};

export default SettingsScreen;