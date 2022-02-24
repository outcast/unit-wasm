FROM ubuntu:focal
RUN apt update && apt-get install -y bash jq
RUN curl -sL https://deb.nodesource.com/setup_16.x -o nodesource_setup.sh
COPY ./js/ /tmp/js
COPY ./rust/pkg/* /tmp/wasm-pkg/
COPY ./job.sh ./tmp
ENTRYPOINT /bin/bash -c /tmp/job.sh