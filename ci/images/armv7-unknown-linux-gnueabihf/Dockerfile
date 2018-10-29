FROM japaric/armv7-unknown-linux-gnueabihf:v0.1.14

RUN dpkg --add-architecture armhf

COPY armhf.list /etc/apt/sources.list.d
COPY amd64.list /etc/apt/sources.list.d

RUN rm -f /etc/apt/sources.list

RUN apt-get update && \
    apt-get install -y --no-install-recommends libmysqlclient-dev:armhf 

