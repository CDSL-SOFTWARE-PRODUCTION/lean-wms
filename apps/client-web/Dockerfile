# Build stage
FROM node:20-slim AS builder
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
RUN corepack prepare pnpm@10.6.4 --activate

WORKDIR /app

# Copy source code (including all workspace packages)
COPY . .

# Install dependencies (including turbo and core packages)
RUN pnpm install --frozen-lockfile

# Install turbo globally to ensure it's available
RUN npm install -g turbo

# Build the app
# Use explicit filter for client-web
RUN turbo build --filter=client-web

# Runtime stage
FROM nginx:alpine AS runtime
COPY --from=builder /app/apps/client-web/dist /usr/share/nginx/html
COPY --from=builder /app/apps/client-web/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
