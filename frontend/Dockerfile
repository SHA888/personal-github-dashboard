# ----------- Build Stage -----------
FROM node:18-alpine as builder
WORKDIR /app

# Install dependencies and build the app
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
RUN npm run build

# ----------- Production Stage -----------
FROM nginx:alpine as production
WORKDIR /usr/share/nginx/html

# Remove default nginx static assets
RUN rm -rf ./*

# Copy built assets from builder
COPY --from=builder /app/dist .

# Copy custom nginx config if you have one
# COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
