import { ReactNode } from 'react';
import { Sidebar } from './Sidebar';
import { ConnectWalletButton } from '../solana/ConnectWalletButton';

interface MainLayoutProps {
  children: ReactNode;
  title: string;
}

export function MainLayout({ children, title }: MainLayoutProps) {
  return (
    <div className="flex min-h-screen bg-gray-900 text-white">
      <Sidebar />
      
      <div className="flex-1 ml-64">
        <div className="p-6">
          <div className="flex justify-between items-center mb-6">
            <h1 className="text-2xl font-semibold">{title}</h1>
            <ConnectWalletButton />
          </div>
          
          {children}
        </div>
      </div>
    </div>
  );
}