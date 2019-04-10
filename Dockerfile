FROM brainpower/rust-cubicle

RUN apt-get update -y && apt-get install libssl-dev pkg-config -y

ENV ROCKET_ADDRESS 0.0.0.0
WORKDIR /root/project
ADD . /root/project/

RUN cargo build
RUN cargo build --tests
RUN rls-build
