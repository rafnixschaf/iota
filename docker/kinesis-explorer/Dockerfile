FROM node:latest as build-stage

COPY . /app

WORKDIR /app

RUN npm install -g pnpm

RUN pnpm install
RUN pnpm explorer build

FROM nginx:stable-alpine as production-stage
COPY --from=build-stage /app/apps/explorer/build /usr/share/nginx/html
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
