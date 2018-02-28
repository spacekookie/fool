FROM japaric/i686-unknown-linux-gnu:latest
MAINTAINER Katharina Fey <kookie@spacekookie.de>

RUN apt-get update
RUN apt-get install -y libncurses5-dev
RUN apt-get install -y libncursesw5-dev:i386
