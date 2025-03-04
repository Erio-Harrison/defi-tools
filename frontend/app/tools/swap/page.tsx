// app/tools/swap/page.tsx
import { MainLayout } from '../../components/layout/MainLayout';

export default function SwapAggregatorPage() {
  return (
    <MainLayout title="交换聚合器">
      <div className="grid grid-cols-12 gap-6">
        <div className="col-span-4 bg-gray-800 rounded-xl border border-gray-700 p-5">
          <div className="font-medium mb-4">交换代币</div>
          
          <div className="space-y-4">
            <div>
              <label className="block text-sm text-gray-400 mb-2">从</label>
              <div className="flex gap-2">
                <select className="flex-1 px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white">
                  <option value="SOL">SOL</option>
                  <option value="USDC">USDC</option>
                  <option value="USDT">USDT</option>
                </select>
                <input 
                  type="text" 
                  placeholder="金额" 
                  className="w-32 px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white"
                />
              </div>
            </div>
            
            <div className="flex justify-center">
              <button className="p-2 rounded-full bg-gray-700 hover:bg-gray-600">
                <span className="text-lg">🔄</span>
              </button>
            </div>
            
            <div>
              <label className="block text-sm text-gray-400 mb-2">到</label>
              <div className="flex gap-2">
                <select className="flex-1 px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white">
                  <option value="USDC">USDC</option>
                  <option value="SOL">SOL</option>
                  <option value="USDT">USDT</option>
                </select>
                <input 
                  type="text" 
                  placeholder="金额" 
                  disabled
                  className="w-32 px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white bg-opacity-70"
                />
              </div>
            </div>
            
            <div>
              <label className="block text-sm text-gray-400 mb-2">最大滑点</label>
              <select className="w-full px-4 py-3 bg-gray-900 border border-gray-700 rounded-lg text-white">
                <option value="0.1">0.1%</option>
                <option value="0.5">0.5%</option>
                <option value="1.0">1.0%</option>
                <option value="2.0">2.0%</option>
              </select>
            </div>
            
            <button className="w-full py-3 bg-indigo-600 rounded-lg font-medium">
              寻找最佳价格
            </button>
          </div>
        </div>
        
        <div className="col-span-8 bg-gray-800 rounded-xl border border-gray-700 p-5">
          <div className="flex justify-between items-center mb-4">
            <div className="font-medium">最佳交易路径</div>
            <div className="text-sm text-gray-400">SOL → USDC</div>
          </div>
          
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-gray-700">
                  <th className="text-left py-3 px-4 font-medium text-gray-400">DEX</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-400">路径</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-400">价格</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-400">收到</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-400">操作</th>
                </tr>
              </thead>
              <tbody>
                <tr className="border-b border-gray-700">
                  <td className="py-4 px-4">
                    <div className="flex items-center">
                      <div className="w-8 h-8 rounded-full bg-blue-600 flex items-center justify-center font-bold mr-2">
                        R
                      </div>
                      Raydium
                    </div>
                  </td>
                  <td className="py-4 px-4">SOL → USDC</td>
                  <td className="py-4 px-4">1 SOL = 32.18 USDC</td>
                  <td className="py-4 px-4 font-mono">32.18 USDC</td>
                  <td className="py-4 px-4">
                    <button className="px-3 py-1 bg-indigo-600 rounded text-sm">
                      交换
                    </button>
                  </td>
                </tr>
                <tr className="border-b border-gray-700">
                  <td className="py-4 px-4">
                    <div className="flex items-center">
                      <div className="w-8 h-8 rounded-full bg-cyan-500 flex items-center justify-center font-bold mr-2">
                        O
                      </div>
                      Orca
                    </div>
                  </td>
                  <td className="py-4 px-4">SOL → USDC</td>
                  <td className="py-4 px-4">1 SOL = 32.14 USDC</td>
                  <td className="py-4 px-4 font-mono">32.14 USDC</td>
                  <td className="py-4 px-4">
                    <button className="px-3 py-1 bg-indigo-600 rounded text-sm">
                      交换
                    </button>
                  </td>
                </tr>
                <tr className="border-b border-gray-700">
                  <td className="py-4 px-4">
                    <div className="flex items-center">
                      <div className="w-8 h-8 rounded-full bg-blue-600 flex items-center justify-center font-bold mr-2">
                        R
                      </div>
                      Raydium
                    </div>
                  </td>
                  <td className="py-4 px-4">SOL → USDT → USDC</td>
                  <td className="py-4 px-4">1 SOL = 32.10 USDC</td>
                  <td className="py-4 px-4 font-mono">32.10 USDC</td>
                  <td className="py-4 px-4">
                    <button className="px-3 py-1 bg-indigo-600 rounded text-sm">
                      交换
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
          
          <div className="mt-4 bg-gray-900 rounded-lg p-4 border border-gray-700">
            <div className="flex items-center">
              <div className="w-10 h-10 rounded-full bg-green-500/20 flex items-center justify-center text-green-500 mr-3">
                💡
              </div>
              <div>
                <div className="font-medium mb-1">价格提示</div>
                <div className="text-sm text-gray-400">
                  Raydium目前提供最佳价格，比其他DEX高出0.15%。交易路径已自动优化以最大化收益。
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </MainLayout>
  );
}