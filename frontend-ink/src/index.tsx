#!/usr/bin/env node
import React from 'react';
import { render } from 'ink';
import App from './components/App';

// Set up default environment variables if not provided
process.env.NODE_ENV = process.env.NODE_ENV || 'development';

// Handle CLI arguments
const args = process.argv.slice(2);
if (args.includes('--help') || args.includes('-h')) {
  console.log(`
nok-ink - React Ink frontend for nok virtual office

Usage:
  nok-ink [options]

Options:
  -h, --help     Show this help message
  -v, --version  Show version information
  --homeserver   Specify Matrix homeserver URL (default: http://nok.local:6167)

Examples:
  nok-ink
  nok-ink --homeserver http://localhost:6167

Controls:
  Tab            Switch between panes
  ↑↓ / k/j       Navigate within panes
  Enter          Select item / Join room
  i              Enter input mode (in Messages pane)
  k              Knock user (in Users pane)
  s              Open settings
  q              Quit application
  Esc            Cancel input / Return to navigation

Environment Variables:
  NOK_HOMESERVER     Matrix homeserver URL
  NOK_USERNAME       Default username for login
  NOK_DEVICE_ID      Matrix device ID
`);
  process.exit(0);
}

if (args.includes('--version') || args.includes('-v')) {
  const packageJson = require('../package.json');
  console.log(`nok-ink v${packageJson.version}`);
  process.exit(0);
}

// Initialize the application
const { waitUntilExit } = render(<App />);

// Handle graceful shutdown
process.on('SIGINT', () => {
  process.exit(0);
});

process.on('SIGTERM', () => {
  process.exit(0);
});

// Wait for the app to exit
waitUntilExit().then(() => {
  process.exit(0);
}).catch((error) => {
  console.error('Application error:', error);
  process.exit(1);
});