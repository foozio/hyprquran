FROM rust:1-bookworm
RUN apt-get update && apt-get install -y \
    pkg-config \
    libgtk-4-dev \
    libpango1.0-dev \
    libglib2.0-dev \
    libgdk-pixbuf-2.0-dev \
    libcairo2-dev \
    libgraphene-1.0-dev \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /workspace
CMD ["bash"]
