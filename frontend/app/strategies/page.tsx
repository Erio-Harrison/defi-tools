import { MainLayout } from '../components/layout/MainLayout';
import Link from 'next/link';

export default function StrategiesPage() {
  return (
    <MainLayout title="策略管理">
      <div className="bg-gray-800 rounded-xl border border-gray-700 p-5 mb-6">
        <div className="flex justify-between items-center mb-4">
          <div className="font-medium">活跃策略</div>
          <Link href="/strategies/create" className="text-indigo-400 text-sm">
            创建新策略
          </Link>
        </div>
        
        <div className="space-y-1">
          <StrategyItem 
            name="稳定收益最大化器"
            allocations={[
              { protocol: "Solend", asset: "USDC", percentage: 50 },
              { protocol: "Raydium", asset: "USDC-USDT LP", percentage: 30 },
              { protocol: "Orca", asset: "SOL-USDC LP", percentage: 20 }
            ]}
            apy={9.2}
          />
          
          <StrategyItem 
            name="保守USDC收益器"
            allocations={[
              { protocol: "Solend", asset: "USDC", percentage: 75 },
              { protocol: "Mango", asset: "USDC", percentage: 25 }
            ]}
            apy={5.7}
          />
          
          <StrategyItem 
            name="SOL LP复利器"
            allocations={[
              { protocol: "Raydium", asset: "SOL-USDC LP", percentage: 60 },
              { protocol: "Orca", asset: "SOL-USDT LP", percentage: 40 }
            ]}
            apy={12.8}
          />
        </div>
      </div>
    </MainLayout>
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