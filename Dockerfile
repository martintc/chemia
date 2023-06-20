FROM fedora:38

WORKDIR /app

COPY . /app

RUN dnf -y install rust cargo
