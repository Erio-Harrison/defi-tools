'use client';

import { useState } from 'react';
import { MainLayout } from '../../components/layout/MainLayout';

export default function CreateStrategyPage() {
  // 选项卡状态
  const [activeTab, setActiveTab] = useState('setup');
  
  // 表单状态
  const [strategyName, setStrategyName] = useState('');
  const [investmentAmount, setInvestmentAmount] = useState('');
  
  // 资产分配状态
  const [allocations, setAllocations] = useState([
    { protocol: 'Solend', asset: 'USDC', weight: '50' },
    { protocol: 'Raydium', asset: 'SOL-USDC LP', weight: '50' }
  ]);
  
  // 新分配项
  const [newAllocation, setNewAllocation] = useState({
    protocol: '',
    asset: '',
    weight: ''
  });
  
  // 添加新分配
  const addAllocation = () => {
    if (newAllocation.protocol && newAllocation.asset && newAllocation.weight) {
      setAllocations([...allocations, { ...newAllocation }]);
      setNewAllocation({ protocol: '', asset: '', weight: '' });
    }
  };
  
  // 删除分配
  const removeAllocation = (index: number) => {
    setAllocations(allocations.filter((_, i) => i !== index));
  };
  
  // 提交表单
  const handleSubmit = () => {
    console.log({
      strategyName,
      investmentAmount,
      allocations
    });
    // 这里会添加与链上交互的逻辑
  };

  return (
    <MainLayout title="创建新策略">
      <div className="bg-gray-800 rounded-xl border border-gray-700 p-6">
        <div className="flex space-x-1 border-b border-gray-700 mb-6">
          <div 
            className={`px-5 py-3 relative cursor-pointer ${
              activeTab === 'setup' ? 'text-indigo-400' : 'text-gray-400'
            }`}
            onClick={() => setActiveTab('setup')}
          >
            策略设置
            {activeTab === 'setup' && <div className="absolute bottom-0 left-0 w-full h-0.5 bg-indigo-500"></div>}
          </div>
          <div 
            className={`px-5 py-3 relative cursor-pointer ${
              activeTab === 'risk' ? 'text-indigo-400' : 'text-gray-400'
            }`}
            onClick={() => setActiveTab('risk')}
          >
            风险配置
            {activeTab === 'risk' && <div className="absolute bottom-0 left-0 w-full h-0.5 bg-indigo-500"></div>}
          </div>
          <div 
            className={`px-5 py-3 relative cursor-pointer ${
              activeTab === 'advanced' ? 'text-indigo-400' : 'text-gray-400'
            }`}
            onClick={() => setActiveTab('advanced')}
          >
            高级设置
            {activeTab === 'advanced' && <div className="absolute bottom-0 left-0 w-full h-0.5 bg-indigo-500"></div>}
          </div>
        </div>
        
        {activeTab === 'setup' && (
          <div className="space-y-6">
            <div>
              <label className="block text-sm text-gray-400 mb-2">策略名称</label>
              <input 
                type="text" 
                placeholder="例如: 稳定收益优化器" 
                className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white"
                value={strategyName}
                onChange={(e) => setStrategyName(e.target.value)}
              />
            </div>
            
            <div>
              <label className="block text-sm text-gray-400 mb-2">投资金额</label>
              <input 
                type="text" 
                placeholder="SOL或USDC金额" 
                className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white"
                value={investmentAmount}
                onChange={(e) => setInvestmentAmount(e.target.value)}
              />
            </div>
            
            <div>
              <label className="block text-sm text-gray-400 mb-2">资产分配</label>
              <div className="p-4 border border-gray-700 rounded-lg">
                <div className="flex gap-3 mb-3">
                  <div className="flex-1">
                    <select 
                      className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white"
                      value={newAllocation.protocol}
                      onChange={(e) => setNewAllocation({...newAllocation, protocol: e.target.value})}
                    >
                      <option value="">选择协议</option>
                      <option value="Raydium">Raydium</option>
                      <option value="Solend">Solend</option>
                      <option value="Orca">Orca</option>
                      <option value="Mango Markets">Mango Markets</option>
                    </select>
                  </div>
                  <div className="flex-1">
                    <select 
                      className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white"
                      value={newAllocation.asset}
                      onChange={(e) => setNewAllocation({...newAllocation, asset: e.target.value})}
                    >
                      <option value="">选择资产</option>
                      <option value="SOL">SOL</option>
                      <option value="USDC">USDC</option>
                      <option value="USDT">USDT</option>
                      <option value="SOL-USDC LP">SOL-USDC LP</option>
                    </select>
                  </div>
                  <div className="w-32">
                    <input 
                      type="text" 
                      placeholder="权重 %" 
                      className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white"
                      value={newAllocation.weight}
                      onChange={(e) => setNewAllocation({...newAllocation, weight: e.target.value})}
                    />
                  </div>
                  <div className="flex items-center justify-center w-10">
                    <button className="text-xl" onClick={addAllocation}>➕</button>
                  </div>
                </div>
                
                {allocations.map((allocation, index) => (
                  <div className="flex gap-3 mb-3" key={index}>
                    <div className="flex-1">
                      <select 
                        className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white"
                        value={allocation.protocol}
                        onChange={(e) => {
                          const newAllocations = [...allocations];
                          newAllocations[index].protocol = e.target.value;
                          setAllocations(newAllocations);
                        }}
                      >
                        <option value="Raydium">Raydium</option>
                        <option value="Solend">Solend</option>
                        <option value="Orca">Orca</option>
                        <option value="Mango Markets">Mango Markets</option>
                      </select>
                    </div>
                    <div className="flex-1">
                      <select 
                        className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white"
                        value={allocation.asset}
                        onChange={(e) => {
                          const newAllocations = [...allocations];
                          newAllocations[index].asset = e.target.value;
                          setAllocations(newAllocations);
                        }}
                      >
                        <option value="SOL">SOL</option>
                        <option value="USDC">USDC</option>
                        <option value="USDT">USDT</option>
                        <option value="SOL-USDC LP">SOL-USDC LP</option>
                      </select>
                    </div>
                    <div className="w-32">
                      <input 
                        type="text" 
                        className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white"
                        value={allocation.weight}
                        onChange={(e) => {
                          const newAllocations = [...allocations];
                          newAllocations[index].weight = e.target.value;
                          setAllocations(newAllocations);
                        }}
                      />
                    </div>
                    <div className="flex items-center justify-center w-10">
                      <button 
                        className="text-red-500"
                        onClick={() => removeAllocation(index)}
                      >
                        ❌
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}
        
        {activeTab === 'risk' && (
          <div>
            <div className="space-y-6">
              <div>
                <label className="block text-sm text-gray-400 mb-2">最大偏差</label>
                <select className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white">
                  <option>5%</option>
                  <option>10%</option>
                  <option>15%</option>
                  <option>20%</option>
                </select>
              </div>
              
              <div>
                <label className="block text-sm text-gray-400 mb-2">时间间隔</label>
                <select className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white">
                  <option>每日</option>
                  <option>每周</option>
                  <option>每月</option>
                  <option>每季度</option>
                </select>
              </div>
              
              <div>
                <label className="block text-sm text-gray-400 mb-2">最大滑点</label>
                <select className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white">
                  <option>0.1%</option>
                  <option>0.5%</option>
                  <option>1.0%</option>
                  <option>2.0%</option>
                </select>
              </div>
            </div>
          </div>
        )}
        
        {activeTab === 'advanced' && (
          <div>
            <div className="space-y-6">
              <div>
                <label className="block text-sm text-gray-400 mb-2">自动再平衡</label>
                <div className="flex items-center">
                  <input type="checkbox" className="mr-2" />
                  <span>启用自动再平衡</span>
                </div>
              </div>
              
              <div>
                <label className="block text-sm text-gray-400 mb-2">紧急停止条件</label>
                <select className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white">
                  <option>价格波动超过15%</option>
                  <option>TVL下降超过20%</option>
                  <option>协议风险评分上升</option>
                </select>
              </div>
            </div>
          </div>
        )}
        
        <div className="mt-8 flex justify-end space-x-4">
          <button className="px-4 py-2 border border-gray-600 rounded-lg">
            取消
          </button>
          <button 
            className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 rounded-lg transition-colors"
            onClick={handleSubmit}
          >
            创建策略
          </button>
        </div>
      </div>
    </MainLayout>
  );
}