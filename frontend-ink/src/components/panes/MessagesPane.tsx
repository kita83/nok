import React from 'react';
import { Box, Text } from 'ink';
import { useAppStore } from '@/store/appStore';
import { PaneType, MessageType } from '@/types';

const MessagesPane: React.FC = () => {
  const {
    messages,
    selectedMessageIndex,
    focusedPane,
    isInputMode,
    inputValue,
    currentRoom,
  } = useAppStore();

  const isFocused = focusedPane === PaneType.Messages;
  const borderColor = isFocused ? 'cyan' : 'white';

  const formatTime = (date: Date): string => {
    return date.toLocaleTimeString('en-US', { 
      hour12: false, 
      hour: '2-digit', 
      minute: '2-digit' 
    });
  };

  const getMessageIcon = (type: MessageType): string => {
    switch (type) {
      case MessageType.Knock:
        return '🔔';
      case MessageType.Notice:
        return 'ℹ️';
      case MessageType.Emote:
        return '✨';
      default:
        return '';
    }
  };

  // Filter messages for current room
  const roomMessages = currentRoom 
    ? messages.filter(msg => msg.roomId === currentRoom.id)
    : messages;

  return (
    <Box
      flexDirection="column"
      borderStyle="round"
      borderColor={borderColor}
      padding={1}
      height="100%"
    >
      <Text bold color={isFocused ? 'cyan' : 'white'}>
        Messages {currentRoom ? `(${currentRoom.name})` : ''}
      </Text>
      
      <Box flexDirection="column" flexGrow={1}>
        {roomMessages.length === 0 ? (
          <Text color="gray">No messages</Text>
        ) : (
          roomMessages.map((message, index) => {
            const isSelected = isFocused && !isInputMode && index === selectedMessageIndex;
            const icon = getMessageIcon(message.type);
            const timeStr = formatTime(message.timestamp);
            
            const backgroundColor = isSelected ? 'black' : undefined;
            const textColor = isSelected ? 'cyan' : 'white';
            
            return (
              <Text
                key={message.id}
                color={textColor}
                backgroundColor={backgroundColor}
              >
                {isSelected ? '> ' : '  '}
                [{timeStr}] {icon && `${icon} `}&lt;{message.sender}&gt;: {message.content}
              </Text>
            );
          })
        )}
      </Box>

      {/* Input area when in input mode */}
      {isInputMode && isFocused && (
        <Box borderStyle="single" borderColor="yellow" padding={0}>
          <Text color="yellow">
            &gt; {inputValue}_
          </Text>
        </Box>
      )}
    </Box>
  );
};

export default MessagesPane;