// App.tsx
// Main application component

import React from 'react';
import { WalletIntegration } from './components/WalletIntegration';
import { TradingInterface } from './components/TradingInterface';

export const App: React.FC = () => {
  return (
    <div>
      <h1>SVMivacy DEX</h1>
      <WalletIntegration />
      <TradingInterface />
    </div>
  );
};