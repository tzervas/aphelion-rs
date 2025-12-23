# aphelion-dev Development Environment
# This Dockerfile sets up the aphelion-dev development environment.

# Using Rust Bookworm version 1.73.0 as base image
FROM rust:1.73.0-bookworm AS base

# Set Maintainer Label
LABEL maintainer="Tyler Zervas <albedo.black.1@gmail.com>"

# Set environment variables for the entry script
ENV RUN_APHELION_RS=false \
    ENABLE_CODE_SERVER=false \
    USE_HTTPS=false \
    UPDATE_CERT=false \
    CUSTOM_RC_FILE="" \
    CUSTOM_NU_FILE="" \
    DEBIAN_FRONTEND=noninteractive \
    RUN_APPLICATION=false \
    APP_NAME=aphelion-rs

# Update packages and install dependencies
RUN apt-get update && apt-get install -y \
    build-essential curl wget git ca-certificates \
    zsh fish jq net-tools iptables \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user and switch to it
RUN useradd -ms /bin/bash devuser
USER devuser

# Switch to the home directory and set it as work directory
WORKDIR /home/devuser

# Expose necessary ports (8080 for HTTP, 443 for HTTPS)
EXPOSE 8080 443

# Copy over the project files
COPY . .

# Copy the entry script into the container
COPY entry-script-detailed-comments.sh /usr/local/bin/entry-script.sh
RUN chmod +x /usr/local/bin/entry-script.sh

# Install code-server
RUN curl -fsSL https://code-server.dev/install.sh | sh

# Install Starship prompt
RUN curl -fsSL https://starship.rs/install.sh | bash

# Configure shell prompt and other QoL features
RUN echo 'eval "$(starship init bash)"' >> ~/.bashrc && \
    echo 'eval "$(starship init zsh)"' >> ~/.zshrc && \
    echo 'eval "$(starship init fish)"' >> ~/.config/fish/config.fish

# Healthcheck to ensure the container is running
HEALTHCHECK --interval=30s --timeout=5s CMD curl --fail http://localhost:8080 || exit 1

# Entry script as the default command
CMD ["/usr/local/bin/entry-script.sh"]
