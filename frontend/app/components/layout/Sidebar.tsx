'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';

const NavItem = ({ href, icon, label }: { href: string; icon: string; label: string }) => {
  const pathname = usePathname();
  const isActive = pathname === href;
  
  return (
    <Link 
      href={href} 
      className={`flex items-center p-3 rounded-lg mb-1 transition-colors ${
        isActive 
          ? 'bg-indigo-900/20 text-indigo-400' 
          : 'text-gray-400 hover:bg-white/5 hover:text-white'
      }`}
    >
      <span className="mr-3 text-xl">{icon}</span>
      {label}
    </Link>
  );
};

export function Sidebar() {
  return (
    <div className="w-64 bg-gray-900 border-r border-gray-800 h-screen fixed p-6 flex flex-col">
      <div className="flex items-center mb-8">
        <div className="bg-indigo-600 w-9 h-9 rounded-lg flex items-center justify-center mr-3 font-bold text-lg">
          SY
        </div>
        <div className="text-xl font-semibold text-white">SolanaYield</div>
      </div>
      
      <div className="text-xs uppercase tracking-wider text-gray-500 mt-6 mb-2">概览</div>
      <NavItem href="/dashboard" icon="📊" label="仪表板" />
      <NavItem href="/strategies" icon="📈" label="投资组合" />
      
      <div className="text-xs uppercase tracking-wider text-gray-500 mt-6 mb-2">工具</div>
      <NavItem href="/tools/swap" icon="🔄" label="交换聚合器" />
      <NavItem href="/tools/yield" icon="💰" label="收益优化器" />
      <NavItem href="/tools/risk" icon="🛡️" label="风险评估" />
      
      <div className="text-xs uppercase tracking-wider text-gray-500 mt-6 mb-2">策略</div>
      <NavItem href="/strategies" icon="⚙️" label="管理策略" />
      <NavItem href="/strategies/create" icon="➕" label="创建策略" />
      
      <div className="mt-auto text-center text-xs text-gray-600">
        <div>SolanaYield DeFi工具集</div>
        <div className="mt-1">v0.1.0</div>
      </div>
    </div>
  );
}