FROM --platform=${BUILDPLATFORM} docker.io/library/golang:1.24 as build
ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN apt-get update && apt-get install -y curl clang

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
RUN . "$HOME/.cargo/env"

ENV PATH="/root/.cargo/bin:/usr/local/bin:$PATH"

RUN mkdir /usr/src/controller
WORKDIR /usr/src/controller
COPY . .

ARG features=""
RUN cargo install --locked --features=${features} --path .

FROM --platform=${BUILDPLATFORM} docker.io/library/golang:1.24
WORKDIR /apps
RUN mkdir /.config
COPY --from=build /usr/src/controller/target/release/controller /apps
EXPOSE 8080
ENTRYPOINT ["/apps/controller"]
