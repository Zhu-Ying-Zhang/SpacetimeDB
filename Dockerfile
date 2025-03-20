# Use a base image that supports multi-arch
FROM rust:bookworm

# Install dependencies
RUN apt-get update && apt-get install -y \
      curl \
      ca-certificates \
      binaryen \
      build-essential \
      && rm -rf /var/lib/apt/lists/*

# Install SpacetimeDB
RUN curl -sSfL https://install.spacetimedb.com | bash -s -- --yes
ENV PATH="/home/spacetime/.local/bin:${PATH}"

# Set working directory
WORKDIR /app

# Expose the necessary port
EXPOSE 3000

# Define the entrypoint
ENTRYPOINT ["spacetime"]

