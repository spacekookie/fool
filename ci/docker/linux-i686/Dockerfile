FROM japaric/i686-unknown-linux-gnu:latest
MAINTAINER Katharina Fey <kookie@spacekookie.de>

RUN apt-get update
RUN apt-get install -y libncurses5-dev  libncursesw5-dev git

# Enable multilib
# RUN sed -i '/#\[multilib\]/,/#Include = \/etc\/pacman.d\/mirrorlist/ s/#//' /etc/pacman.conf && \
#     sed -i '/#\[multilib\]/,/#Include = \/etc\/pacman.d\/mirrorlist/ s/#//' /etc/pacman.conf && \
#     sed -i 's/#\[multilib\]/\[multilib\]/g' /etc/pacman.conf

# RUN pacman -Syyu --noconfirm

# RUN apt-get update