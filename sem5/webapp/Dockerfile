FROM node:22-alpine

RUN apk add --no-cache python3 make g++ sqlite-dev

WORKDIR /app

COPY package*.json ./

RUN npm install --legacy-peer-deps #--build-from-source=better-sqlite3

COPY . .

EXPOSE 5000

CMD ["npx", "tsx", "src/backend/server/server.ts"]
