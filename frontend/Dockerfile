FROM node:14.9 as builder

WORKDIR /frontend

RUN mkdir src

COPY src /frontend/src
COPY package.json /frontend
COPY tsconfig.json /frontend
COPY webpack.shared.js /frontend
COPY webpack.prod.js /frontend

RUN yarn && yarn build

# --------

FROM nginx:1.19.2-alpine as echo-nginx

RUN mkdir /usr/share/nginx/html/assets
COPY --from=builder /frontend/public/assets/index.js /usr/share/nginx/html/assets/index.js
COPY public/index.html /usr/share/nginx/html
COPY .nginx/default.conf /etc/nginx/conf.d/default.conf

EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]
