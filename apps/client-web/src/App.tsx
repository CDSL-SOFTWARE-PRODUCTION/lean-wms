import { QueryClientProvider } from '@tanstack/react-query';
import { queryClient } from './utils/rspc';
import { HealthCheck } from './components/HealthCheck';
import { AuthProvider } from './providers/AuthProvider';
import './App.css';

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <AuthProvider>
        <div className="min-h-screen bg-slate-50">
          <header className="bg-slate-900 text-white p-6 shadow-md">
            <h1 className="text-2xl font-bold">Lean WMS Dashboard</h1>
          </header>

          <main className="container mx-auto py-8 px-4">
            <HealthCheck />
            <div className="mt-8 grid grid-cols-1 md:grid-cols-3 gap-6">
              <div className="bg-white p-6 border rounded-xl shadow-sm">
                <h2 className="text-lg font-bold mb-2">Welcome to Lean WMS</h2>
                <p className="text-slate-600 text-sm">
                  Management Dashboard for warehouse operations.
                </p>
              </div>
            </div>
          </main>
        </div>
      </AuthProvider>
    </QueryClientProvider>
  );
}

export default App;
