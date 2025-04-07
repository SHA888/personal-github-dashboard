import React, { useEffect, useState } from "react";
import { useWebSocket } from "../services/websocket";

interface WebSocketProviderProps {
  children: React.ReactNode;
}

export const WebSocketProvider: React.FC<WebSocketProviderProps> = ({
  children,
}) => {
  const { connect, disconnect, isConnected, error } = useWebSocket();
  const [retryCount, setRetryCount] = useState(0);
  const maxRetries = 3;

  useEffect(() => {
    connect();

    const handleVisibilityChange = () => {
      if (document.visibilityState === "visible" && !isConnected) {
        connect();
      }
    };

    document.addEventListener("visibilitychange", handleVisibilityChange);

    return () => {
      disconnect();
      document.removeEventListener("visibilitychange", handleVisibilityChange);
    };
  }, [connect, disconnect, isConnected]);

  useEffect(() => {
    if (error && retryCount < maxRetries) {
      const timer = setTimeout(() => {
        setRetryCount((prev) => prev + 1);
        connect();
      }, 5000); // Retry every 5 seconds

      return () => clearTimeout(timer);
    }
  }, [error, retryCount, connect]);

  return (
    <div>
      {!isConnected && (
        <div className="fixed top-0 left-0 right-0 bg-yellow-100 text-yellow-800 p-2 text-center">
          {error ? (
            <span>
              WebSocket connection error.{" "}
              {retryCount < maxRetries ? "Retrying..." : "Max retries reached."}
            </span>
          ) : (
            "Connecting to WebSocket..."
          )}
        </div>
      )}
      {children}
    </div>
  );
};
