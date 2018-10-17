FROM japaric/aarch64-unknown-linux-gnu:v0.1.14

RUN dpkg --add-architecture arm64 

COPY arm64.list /etc/apt/sources.list.d
COPY amd64.list /etc/apt/sources.list.d

RUN rm -f /etc/apt/sources.list

RUN apt-get update && \
    apt-get install -y --no-install-recommends libmysqlclient-dev:arm64 
