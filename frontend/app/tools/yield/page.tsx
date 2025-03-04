'use client';

import { SetStateAction, useState } from 'react';
import { MainLayout } from '../../components/layout/MainLayout';

export default function YieldOptimizerPage() {
  const [riskLevel, setRiskLevel] = useState(3);
  const [asset, setAsset] = useState('USDC');
  const [amount, setAmount] = useState('');

  // Handler functions
  const handleAssetChange = (e: { target: { value: SetStateAction<string>; }; }) => setAsset(e.target.value);
  const handleAmountChange = (e: { target: { value: SetStateAction<string>; }; }) => setAmount(e.target.value);
  const handleSearch = () => {
    console.log('Searching with:', { asset, amount, riskLevel });
    // Add your search logic here
  };

  return (
    <MainLayout title="收益率优化器">
      <div className="grid grid-cols-12 gap-6">
        {/* Search Panel */}
        <div className="col-span-4 bg-gray-800 rounded-xl border border-gray-700 p-5">
          <div className="font-medium mb-4">收益率搜索</div>
          
          <div className="space-y-4">
            <div>
              <label className="block text-sm text-gray-400 mb-2">资产</label>
              <select 
                value={asset}
                onChange={handleAssetChange}
                className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white"
              >
                <option value="SOL">SOL</option>
                <option value="USDC">USDC</option>
                <option value="USDT">USDT</option>
              </select>
            </div>
            
            <div>
              <label className="block text-sm text-gray-400 mb-2">金额</label>
              <input 
                type="text" 
                value={amount}
                onChange={handleAmountChange}
                placeholder="输入金额" 
                className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white"
              />
            </div>
            
            <div>
              <label className="block text-sm text-gray-400 mb-2">风险容忍度</label>
              <div className="flex items-center">
                <span className="mr-2">低</span>
                <input 
                  type="range" 
                  min="1" 
                  max="5" 
                  value={riskLevel} 
                  onChange={(e) => setRiskLevel(parseInt(e.target.value))} 
                  className="flex-1" 
                />
                <span className="ml-2">高</span>
              </div>
            </div>
            
            <button 
              onClick={handleSearch}
              className="w-full py-3 bg-indigo-600 rounded-lg font-medium hover:bg-indigo-700 transition-colors"
            >
              查找最佳收益
            </button>
          </div>
        </div>
        
        {/* Results Panel */}
        <div className="col-span-8 bg-gray-800 rounded-xl border border-gray-700 p-5">
          <div className="flex justify-between items-center mb-4">
            <div className="font-medium">收益率比较</div>
            <div className="text-sm text-gray-400">显示{asset}选项</div>
          </div>
          
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-gray-700">
                  <th className="text-left py-3 px-4 font-medium text-gray-400">协议</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-400">策略</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-400">APY</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-400">风险评分</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-400">操作</th>
                </tr>
              </thead>
              <tbody>
                <tr className="border-b border-gray-700 hover:bg-gray-700 transition-colors">
                  <td className="py-4 px-4">
                    <div className="flex items-center">
                      <div className="w-8 h-8 rounded-full bg-purple-600 flex items-center justify-center font-bold mr-2">
                        S
                      </div>
                      Solend
                    </div>
                  </td>
                  <td className="py-4 px-4">Supply {asset}</td>
                  <td className="py-4 px-4 text-green-500 font-semibold">5.8%</td>
                  <td className="py-4 px-4">
                    <div className="w-24 h-2 bg-gradient-to-r from-green-500 via-yellow-500 to-red-500 rounded relative">
                      <div 
                        className="absolute top-1/2 -translate-y-1/2 w-3 h-3 bg-white rounded-full"
                        style={{ left: `${(2 / 5) * 100}%` }}
                      ></div>
                    </div>
                  </td>
                  <td className="py-4 px-4">
                    <button className="px-3 py-1 bg-indigo-600 rounded text-sm hover:bg-indigo-700 transition-colors">
                      存款
                    </button>
                  </td>
                </tr>
                {/* You can add more rows here with similar structure */}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </MainLayout>
  );
}