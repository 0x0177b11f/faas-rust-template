FROM openfaas/of-watchdog:0.7.6 as watchdog

FROM rust:1.41.1-slim-buster as builder

WORKDIR /usr/src/

# Copy all the sources
COPY function ./function
COPY main ./main

RUN cd main && cargo install --path .

FROM debian:buster-slim as runner

# Install packages and add non-root user
RUN apt-get update -y \
    && apt-get install -y curl ca-certificates \
    && apt-get clean \
    && useradd -ms /bin/bash app

ENV USER=app

WORKDIR /home/app

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

COPY --from=builder /usr/local/cargo/bin/main /home/app/main
RUN chmod +x /home/app/main

# Set up watchdog for HTTP mode
ENV fprocess="./main"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

USER app

CMD ["fwatchdog"]
