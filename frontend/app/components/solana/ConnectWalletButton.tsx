'use client';

import { useWallet } from '@solana/wallet-adapter-react';
import { useWalletModal } from '@solana/wallet-adapter-react-ui';

export function ConnectWalletButton() {
  const { connected, publicKey } = useWallet();
  const { setVisible } = useWalletModal();

  const handleClick = () => {
    if (!connected) {
      setVisible(true);
    }
  };

  return (
    <button 
      onClick={handleClick}
      className="flex items-center bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg py-2 px-4 transition-colors"
    >
      <span className="mr-2">🔌</span>
      {connected ? 
        `已连接: ${publicKey?.toString().slice(0, 4)}...${publicKey?.toString().slice(-4)}` : 
        '连接钱包'}
    </button>
  );
}