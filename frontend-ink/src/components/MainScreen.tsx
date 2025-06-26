import React from 'react';
import { Box } from 'ink';
import RoomsPane from './panes/RoomsPane';
import UsersPane from './panes/UsersPane';
import MessagesPane from './panes/MessagesPane';
import StatusPane from './panes/StatusPane';

const MainScreen: React.FC = () => {
  return (
    <Box height="100%">
      {/* Left column - 40% width */}
      <Box flexDirection="column" width="40%">
        {/* Rooms pane - fixed height */}
        <Box height={8}>
          <RoomsPane />
        </Box>
        
        {/* Users pane - fixed height */}
        <Box height={8}>
          <UsersPane />
        </Box>
        
        {/* Messages pane - remaining height */}
        <Box flexGrow={1}>
          <MessagesPane />
        </Box>
      </Box>

      {/* Right column - 60% width */}
      <Box width="60%">
        <StatusPane />
      </Box>
    </Box>
  );
};

export default MainScreen;