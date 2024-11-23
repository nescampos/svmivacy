// WalletIntegration.tsx
// Component for integrating Solana wallets like Phantom and Solflare

import React from 'react';

export const WalletIntegration: React.FC = () => {
  return (
    <div>
      <button onClick={() => console.log('Connect wallet')}>
        Connect Wallet
      </button>
    </div>
  );
};