FROM ubuntu:24.04

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        unzip \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fL \
    https://github.com/ronilan/incredible/releases/latest/download/incredible-terminal-linux.zip \
    -o /tmp/incredible.zip \
    && unzip -o /tmp/incredible.zip -d /usr/local/bin \
    && rm /tmp/incredible.zip \
    && chmod +x /usr/local/bin/incredible

CMD ["/bin/bash", "-i"]
