FROM archlinux/base
MAINTAINER Katharina Fey <kookie@spacekookie.de>

# Enable multilib
RUN sed -i '/#\[multilib\]/,/#Include = \/etc\/pacman.d\/mirrorlist/ s/#//' /etc/pacman.conf && \
    sed -i '/#\[multilib\]/,/#Include = \/etc\/pacman.d\/mirrorlist/ s/#//' /etc/pacman.conf && \
    sed -i 's/#\[multilib\]/\[multilib\]/g' /etc/pacman.conf

RUN pacman -Syyu --noconfirm

# RUN apt-get update
# RUN apt-get install -y libncurses5-dev
# RUN apt-get install -y libncursesw5-dev
