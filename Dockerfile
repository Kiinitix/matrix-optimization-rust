FROM rust:1.70

# Install minimal tools needed for perf to build without optional features
RUN apt-get update && apt-get install -y \
    build-essential \
    flex \
    bison \
    libelf-dev \
    curl \
    git \
    zlib1g-dev \
    libtraceevent-dev \
    python3

# Clone and build perf
RUN git clone --depth 1 https://github.com/torvalds/linux.git /usr/src/linux && \
    cd /usr/src/linux/tools/perf && \
    make NO_LIBTRACEEVENT=1 NO_SDT=1 NO_LIBBABELTRACE=1 NO_LIBCAPSTONE=1 NO_LIBPYTHON=1 NO_LIBPFM=1 -j$(nproc) && \
    cp perf /usr/local/bin/

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["bash", "scripts/run_benchmarks.sh"]




# docker build -t matrix-benchmark .
# docker run --rm --cap-add=SYS_ADMIN --security-opt seccomp=unconfined matrix-benchmark
