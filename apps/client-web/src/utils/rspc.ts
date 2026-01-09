import { createClient, FetchTransport } from "@rspc/client";
import { QueryClient } from "@tanstack/react-query";

// Import generated types from shared core
import type { Procedures } from "@lean-wms/core"; 

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: false,
      refetchOnWindowFocus: false,
    },
  },
});

export const rspc = createClient<Procedures>({
  transport: new FetchTransport("http://localhost:3000/rspc"),
});