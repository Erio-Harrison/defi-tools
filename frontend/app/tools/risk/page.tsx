'use client';

import { useState } from 'react';
import { MainLayout } from '../../components/layout/MainLayout';

export default function RiskAssessmentPage() {
  const [applying, setApplying] = useState(false);
  const [recommendations, setRecommendations] = useState([
    {
      title: "多样化资产配置",
      description: "您的投资组合高度集中在稳定币上。考虑添加SOL或其他资产以提高多样性。",
      applied: false
    },
    {
      title: "减少Mango敞口",
      description: "Mango Markets具有较高的风险特征。考虑将其分配降低到投资组合的20%以下。",
      applied: false
    },
    {
      title: "设置自动再平衡",
      description: "配置自动再平衡，以在市场条件变化时维持目标风险状况。",
      applied: false
    }
  ]);
  
  const handleApplyRecommendations = () => {
    setApplying(true);
    
    // 模拟应用过程
    setTimeout(() => {
      setRecommendations(prev => prev.map(rec => ({...rec, applied: true})));
      setApplying(false);
    }, 1000);
  };

  return (
    <MainLayout title="风险评估">
      <div className="grid grid-cols-12 gap-6">
        <div className="col-span-12 bg-gray-800 rounded-xl border border-gray-700 p-5 mb-6">
          <div className="font-medium mb-4">协议风险概览</div>
          
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-gray-700">
                  <th className="text-left py-3 px-4 font-medium text-gray-400">协议</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-400">TVL</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-400">审计状态</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-400">漏洞历史</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-400">风险评分</th>
                </tr>
              </thead>
              <tbody>
                <tr className="border-b border-gray-700">
                  <td className="py-4 px-4">
                    <div className="flex items-center">
                      <div className="w-8 h-8 rounded-full bg-purple-600 flex items-center justify-center font-bold mr-2">
                        S
                      </div>
                      Solend
                    </div>
                  </td>
                  <td className="py-4 px-4">$285M</td>
                  <td className="py-4 px-4">
                    <span className="text-green-500">已审计 ✓</span>
                  </td>
                  <td className="py-4 px-4">无</td>
                  <td className="py-4 px-4">
                    <div className="w-24 h-2 bg-gradient-to-r from-green-500 via-yellow-500 to-red-500 rounded relative">
                      <div className="absolute top-1/2 left-[20%] -translate-y-1/2 w-3 h-3 bg-white rounded-full"></div>
                    </div>
                  </td>
                </tr>
                {/* 其他协议行... */}
              </tbody>
            </table>
          </div>
        </div>
        
        <div className="col-span-6 bg-gray-800 rounded-xl border border-gray-700 p-5">
          <div className="font-medium mb-4">投资组合风险因素</div>
          
          <div className="space-y-4">
            <div>
              <div className="flex justify-between mb-1">
                <div>协议多样性：</div>
                <div className="text-green-500">低风险</div>
              </div>
              <div className="h-2 bg-gradient-to-r from-green-500 via-yellow-500 to-red-500 rounded relative">
                <div className="absolute top-1/2 left-[15%] -translate-y-1/2 w-3 h-3 bg-white rounded-full"></div>
              </div>
            </div>
            
            {/* 其他风险因素... */}
          </div>
        </div>
        
        <div className="col-span-6 bg-gray-800 rounded-xl border border-gray-700 p-5">
          <div className="font-medium mb-4">风险缓解建议</div>
          
          <div className="space-y-4">
            {recommendations.map((rec, index) => (
              <div key={index} className={rec.applied ? "opacity-50" : ""}>
                <div className="font-medium mb-1 flex items-center">
                  {rec.title}
                  {rec.applied && <span className="ml-2 text-green-500 text-sm">✓ 已应用</span>}
                </div>
                <div className="text-sm text-gray-400">
                  {rec.description}
                </div>
              </div>
            ))}
            
            <button 
              className={`w-full py-3 rounded-lg font-medium mt-4 ${
                applying 
                  ? "bg-indigo-800 cursor-wait" 
                  : recommendations.every(r => r.applied) 
                    ? "bg-gray-600 cursor-not-allowed"
                    : "bg-indigo-600 hover:bg-indigo-700"
              }`}
              onClick={handleApplyRecommendations}
              disabled={applying || recommendations.every(r => r.applied)}
            >
              {applying ? "应用中..." : recommendations.every(r => r.applied) ? "已应用所有建议" : "应用建议"}
            </button>
          </div>
        </div>
      </div>
    </MainLayout>
  );
}