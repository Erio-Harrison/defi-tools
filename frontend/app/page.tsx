// app/page.tsx
import { redirect } from 'next/navigation';

// 在服务器端进行重定向
export const metadata = {
  title: 'Home Page',
};

export default function Home() {
  redirect('/dashboard');
}
