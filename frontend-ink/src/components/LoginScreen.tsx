import React, { useState } from 'react';
import { Box, Text, useInput, Spacer } from 'ink';
import { useAppStore } from '@/store/appStore';
import { useMatrix } from '@/hooks/useMatrix';
import { ViewMode, ConnectionStatus } from '@/types';

const LoginScreen: React.FC = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [focusedField, setFocusedField] = useState<'username' | 'password'>('username');
  const [isLoggingIn, setIsLoggingIn] = useState(false);

  const {
    setCurrentView,
    connectionStatus,
    error,
  } = useAppStore();

  const { login } = useMatrix();

  useInput((input, key) => {
    if (key.tab) {
      setFocusedField(focusedField === 'username' ? 'password' : 'username');
    } else if (key.return) {
      handleLogin();
    } else if (key.escape) {
      // Exit app
      process.exit(0);
    } else if (key.backspace) {
      if (focusedField === 'username') {
        setUsername(username.slice(0, -1));
      } else {
        setPassword(password.slice(0, -1));
      }
    } else if (input && !key.ctrl && !key.meta) {
      if (focusedField === 'username') {
        setUsername(username + input);
      } else {
        setPassword(password + input);
      }
    }
  });

  const handleLogin = async () => {
    if (!username || !password) {
      return;
    }

    if (isLoggingIn) return;

    setIsLoggingIn(true);

    try {
      const success = await login(username, password);
      if (success) {
        setCurrentView(ViewMode.Main);
      }
    } finally {
      setIsLoggingIn(false);
    }
  };

  return (
    <Box flexDirection="column" padding={2}>
      <Box flexDirection="column" borderStyle="round" borderColor="cyan" padding={1}>
        <Text color="cyan" bold>
          nok Matrix Login
        </Text>
        <Text>
          Welcome to nok Matrix Edition!
        </Text>
        <Text>
          Please enter your Matrix credentials:
        </Text>
      </Box>

      <Spacer />

      <Box flexDirection="column" borderStyle="round" borderColor={focusedField === 'username' ? 'yellow' : 'white'} padding={1}>
        <Text>Username:</Text>
        <Text>
          {username}
          {focusedField === 'username' && '_'}
        </Text>
      </Box>

      <Box flexDirection="column" borderStyle="round" borderColor={focusedField === 'password' ? 'yellow' : 'white'} padding={1}>
        <Text>Password:</Text>
        <Text>
          {'*'.repeat(password.length)}
          {focusedField === 'password' && '_'}
        </Text>
      </Box>

      <Box flexDirection="column" borderStyle="round" borderColor="green" padding={1}>
        <Text color="green">
          {isLoggingIn ? 'Logging in...' : 'Press Enter to login'}
        </Text>
        <Text color="green">
          Status: {connectionStatus}
        </Text>
      </Box>

      {error && (
        <Box flexDirection="column" borderStyle="round" borderColor="red" padding={1}>
          <Text color="red">Error: {error}</Text>
        </Box>
      )}

      <Box flexDirection="column" borderStyle="round" borderColor="gray" padding={1}>
        <Text color="gray">Help:</Text>
        <Text color="gray">• Tab: Switch between fields</Text>
        <Text color="gray">• Enter: Login</Text>
        <Text color="gray">• Esc: Exit</Text>
        <Text color="gray">Example: test1 / demo1234</Text>
      </Box>
    </Box>
  );
};

export default LoginScreen;