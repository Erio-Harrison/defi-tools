'use client';

import { useState } from 'react';
import { MainLayout } from '../components/layout/MainLayout';

export default function DashboardPage() {
  const [timeRange, setTimeRange] = useState('30d');
  const [portfolioValue, setPortfolioValue] = useState(12452.8);
  const [strategies, setStrategies] = useState([
    {
      name: "稳定收益最大化器",
      allocations: [
        { protocol: "Solend", asset: "USDC", percentage: 50 },
        { protocol: "Raydium", asset: "USDC-USDT LP", percentage: 30 },
        { protocol: "Orca", asset: "SOL-USDC LP", percentage: 20 }
      ],
      apy: 9.2
    },
    {
      name: "保守USDC收益器",
      allocations: [
        { protocol: "Solend", asset: "USDC", percentage: 75 },
        { protocol: "Mango", asset: "USDC", percentage: 25 }
      ],
      apy: 5.7
    },
    {
      name: "SOL LP复利器",
      allocations: [
        { protocol: "Raydium", asset: "SOL-USDC LP", percentage: 60 },
        { protocol: "Orca", asset: "SOL-USDT LP", percentage: 40 }
      ],
      apy: 12.8
    }
  ]);

  return (
    <MainLayout title="仪表板">
      <div className="grid grid-cols-4 gap-5 mb-6">
        <StatCard title="总资产价值" value={`$${portfolioValue.toLocaleString()}`} change="+5.2%" positive={true} />
        <StatCard title="活跃策略" value={strategies.length.toString()} description="上次创建于2天前" />
        <StatCard title="估计APY" value="8.4%" change="+0.6%" positive={true} />
        <StatCard title="风险评分" value="23/100" description="低风险投资组合" />
      </div>
      
      <div className="grid grid-cols-12 gap-6">
        <div className="col-span-9 bg-gray-800 rounded-xl border border-gray-700 p-5">
          <div className="flex justify-between items-center mb-4">
            <div className="font-medium">投资组合表现</div>
            <div className="relative">
              <select 
                className="appearance-none bg-transparent text-indigo-400 cursor-pointer pr-6"
                value={timeRange}
                onChange={(e) => setTimeRange(e.target.value)}
              >
                <option value="7d">最近7天</option>
                <option value="30d">最近30天</option>
                <option value="90d">最近90天</option>
                <option value="1y">最近1年</option>
              </select>
              <span className="absolute right-0 top-1/2 -translate-y-1/2 pointer-events-none text-indigo-400">↓</span>
            </div>
          </div>
          <div className="h-64 flex items-center justify-center text-gray-500">
            图表将在这里显示 - {timeRange} 周期数据
          </div>
        </div>
        
        <div className="col-span-3 bg-gray-800 rounded-xl border border-gray-700 p-5">
          <div className="font-medium mb-4">风险评估</div>
          <div className="text-center mb-4">
            <span className="text-4xl font-semibold text-green-500">23</span>
            <span className="text-gray-500 text-xl">/100</span>
          </div>
          <div className="flex items-center mb-4">
            <span>低</span>
            <div className="flex-1 h-2 mx-2 rounded bg-gradient-to-r from-green-500 via-yellow-500 to-red-500 relative">
              <div className="absolute top-1/2 left-[23%] -translate-y-1/2 w-3 h-3 bg-white rounded-full border-2 border-gray-900"></div>
            </div>
            <span>高</span>
          </div>
          <p className="text-sm text-gray-400 text-center">
            您的投资组合具有保守的风险特征和良好的多样化。
          </p>
          <div className="text-center mt-4">
            <button className="px-3 py-1 text-sm bg-transparent border border-gray-600 rounded hover:bg-gray-700 transition-colors">
              查看详情
            </button>
          </div>
        </div>
      </div>
      
      <div className="mt-6 bg-gray-800 rounded-xl border border-gray-700 p-5">
        <div className="flex justify-between items-center mb-4">
          <div className="font-medium">活跃策略</div>
          <button className="text-indigo-400 text-sm hover:text-indigo-300 transition-colors">
            创建新策略
          </button>
        </div>
        
        <div className="space-y-1">
          {strategies.map((strategy, index) => (
            <StrategyItem 
              key={index}
              name={strategy.name}
              allocations={strategy.allocations}
              apy={strategy.apy}
            />
          ))}
        </div>
      </div>
    </MainLayout>
  );
}

function StatCard({ 
  title, 
  value, 
  change, 
  description,
  positive 
}: { 
  title: string; 
  value: string; 
  change?: string; 
  description?: string;
  positive?: boolean 
}) {
  return (
    <div className="bg-gray-800 rounded-xl border border-gray-700 p-5">
      <div className="text-sm text-gray-400 mb-2">{title}</div>
      <div className="text-2xl font-semibold mb-1">{value}</div>
      {change && (
        <div className={`text-sm ${positive ? 'text-green-500' : 'text-red-500'}`}>
          {positive ? '↗️' : '↘️'} {change} 过去24小时
        </div>
      )}
      {description && <div className="text-sm text-gray-400">{description}</div>}
    </div>
  );
}

function StrategyItem({ 
  name, 
  allocations, 
  apy 
}: { 
  name: string; 
  allocations: { protocol: string; asset: string; percentage: number }[]; 
  apy: number; 
}) {
  const getProtocolColor = (protocol: string) => {
    const colors: Record<string, string> = {
      "Raydium": "bg-blue-600",
      "Solend": "bg-purple-600",
      "Orca": "bg-cyan-500",
      "Mango": "bg-yellow-500"
    };
    return colors[protocol] || "bg-gray-500";
  };
  
  return (
    <div className="flex items-center justify-between p-4 border-b border-gray-700 last:border-0">
      <div className="font-medium">{name}</div>
      
      <div className="flex items-center">
        {allocations.map((item, i) => (
          <div key={i} className="flex items-center mr-4 bg-white/10 px-2 py-1 rounded text-xs">
            <div className={`w-6 h-6 rounded-full ${getProtocolColor(item.protocol)} flex items-center justify-center mr-2 text-xs font-bold`}>
              {item.protocol[0]}
            </div>
            {item.protocol} {item.percentage}%
          </div>
        ))}
      </div>
      
      <div className="flex items-center">
        <div className="font-semibold text-green-500 mr-4">+{apy}% APY</div>
        <button className="px-3 py-1 bg-transparent border border-gray-600 rounded text-sm hover:bg-gray-700 transition-colors">
          详情
        </button>
      </div>
    </div>
  );
}