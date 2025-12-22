import "../App.css";
import { useEffect, useState } from "react";
import { Navigate } from "react-router";
import { store } from "@/utils/token";
import { SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/SideBar/app-sidebar";
import { User } from "types/models/User";
import { invoke } from "@tauri-apps/api/core";

export default function Home() {
  const [token, setToken] = useState<boolean>(false);
  const [loading, setLoading] = useState(true);
  const [userInfo, setUserInfo] = useState<User | null>(null);

  useEffect(() => {
    const fetchToken = async () => {
      try {
        const token = await store.get<string>("token");

        setToken(!!token);
        const user: User = await invoke("get_user_info", { token });
        setUserInfo(user);
      } catch (err) {
        console.error(err);
        setToken(false);
      } finally {
        setLoading(false);
      }
    };

    fetchToken();
  }, []);

  if (loading)
    return (
      <div className="w-full h-dvh flex justify-center items-center text-3xl">
        Loading...
      </div>
    );
  if (!token) return <Navigate to="/login" replace />;

  return (
    <SidebarProvider>
      <div className="w-full min-h-dvh flex items-center bg-black text-white p-5">
        <AppSidebar userInfo={userInfo!} />
      </div>
    </SidebarProvider>
  );
}
