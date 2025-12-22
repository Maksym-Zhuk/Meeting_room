import "../App.css";
import { useEffect, useState } from "react";
import { Navigate } from "react-router";
import { store } from "@/utils/token";

export default function Home() {
  const [token, setToken] = useState<boolean>(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchToken = async () => {
      try {
        const token = await store.get<string>("token");

        setToken(!!token);
      } catch (err) {
        console.error(err);
        setToken(false);
      } finally {
        setLoading(false);
      }
    };

    fetchToken();
  }, []);

  if (loading) return <div>Loading...</div>;
  if (!token) return <Navigate to="/register" replace />;

  return (
    <div>
      <h1>Home</h1>
      <p>Welcome! You are logged in.</p>
    </div>
  );
}
