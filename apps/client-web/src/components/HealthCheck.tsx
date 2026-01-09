import { rspc } from "../utils/rspc";
import { useQuery } from "@tanstack/react-query";

export const HealthCheck = () => {
  const { data: version, isLoading: loadingVersion } = useQuery({
    queryKey: ["version"],
    queryFn: () => rspc.query(["version"]),
  });

  const { data: status, isLoading: loadingStatus, error } = useQuery({
    queryKey: ["health_check"],
    queryFn: () => rspc.query(["health_check"]),
  });

  return (
    <div className="p-4 m-4 border rounded-lg bg-slate-50 shadow-sm">
      <h2 className="text-xl font-bold mb-4 text-slate-800">System Health Check</h2>
      
      <div className="space-y-2">
        <div className="flex items-center">
          <span className="font-semibold w-32">API Version:</span>
          {loadingVersion ? (
            <span className="text-slate-400 animate-pulse">Checking...</span>
          ) : (
            <span className="text-blue-600 font-mono">{version}</span>
          )}
        </div>

        <div className="flex items-center">
          <span className="font-semibold w-32">Backend Status:</span>
          {loadingStatus ? (
            <span className="text-slate-400 animate-pulse">Connecting...</span>
          ) : error ? (
            <span className="text-red-500 font-medium">❌ Connection Failed</span>
          ) : (
            <span className="text-green-600 font-medium">✅ {status}</span>
          )}
        </div>
      </div>

      {error && (
        <div className="mt-4 p-2 text-xs bg-red-100 text-red-700 rounded">
          Please make sure the backend API is running on port 3000
        </div>
      )}
    </div>
  );
};
