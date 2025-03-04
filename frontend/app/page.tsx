import { redirect } from 'next/navigation';

// 从根路径重定向到仪表板页面
export default function Home() {
  redirect('/dashboard');
}