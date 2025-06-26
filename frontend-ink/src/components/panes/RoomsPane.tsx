import React from 'react';
import { Box, Text } from 'ink';
import { useAppStore } from '@/store/appStore';
import { PaneType } from '@/types';

const RoomsPane: React.FC = () => {
  const {
    rooms,
    currentRoom,
    selectedRoomIndex,
    focusedPane,
  } = useAppStore();

  const isFocused = focusedPane === PaneType.Rooms;
  const borderColor = isFocused ? 'cyan' : 'white';

  return (
    <Box
      flexDirection="column"
      borderStyle="round"
      borderColor={borderColor}
      padding={1}
      height="100%"
    >
      <Text bold color={isFocused ? 'cyan' : 'white'}>
        Rooms
      </Text>
      
      <Box flexDirection="column" flexGrow={1}>
        {rooms.length === 0 ? (
          <Text color="gray">No rooms available</Text>
        ) : (
          rooms.map((room, index) => {
            const isSelected = isFocused && index === selectedRoomIndex;
            const isCurrent = currentRoom?.id === room.id;
            
            let prefix = '  ';
            if (isCurrent) prefix = '* ';
            if (isSelected) prefix = '> ';
            
            const color = isCurrent ? 'yellow' : isSelected ? 'cyan' : 'white';
            const backgroundColor = isSelected ? 'black' : undefined;
            
            return (
              <Text
                key={room.id}
                color={color}
                backgroundColor={backgroundColor}
              >
                {prefix}{room.name}
              </Text>
            );
          })
        )}
      </Box>
    </Box>
  );
};

export default RoomsPane;