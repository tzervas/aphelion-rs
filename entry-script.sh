#!/bin/bash

# aphelion-dev Docker Entry Script
# Author: Tyler Zervas (albedo_black)
# This script serves as the entry point for the aphelion-dev Docker container.
# It is responsible for setting up the development environment and running
# the $APP_NAME application based on various flags and configurations.

# Function: is_port_available
# Description: Checks if a given port is available on the machine.
# Parameters: Port number to check.
is_port_available() {
    ! netstat -ltnp | grep -wq ":$1"
}

# Function: handle_exception
# Description: Handles exceptions and errors by printing an error message and exiting.
# Parameters: Error message to display.
handle_exception() {
    echo "Error: $1"
    exit 1
}

# Generate or update SSL certificate for dev.aphelion.localhost
# This step is conditional on both the USE_HTTPS and UPDATE_CERT flags.
if [ "$USE_HTTPS" = "true" ] && [ "$UPDATE_CERT" = "true" ]; then
    openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout /etc/ssl/private/dev-aphelion-localhost.key -out /etc/ssl/certs/dev-aphelion-localhost.crt -subj "/CN=dev.aphelion.localhost" || handle_exception "Failed to generate SSL certificate"
fi

# Determine the port for the development server based on whether HTTPS is used.
if [ "$USE_HTTPS" = "true" ]; then
    PORT=443
else
    PORT=8080
fi

# Dynamically find an available port if the default port is taken.
while ! is_port_available "$PORT"; do
    PORT=$((PORT + 1))
done

# Update localhost URL based on the chosen port.
LOCALHOST_URL="dev.aphelion.localhost:$PORT"

# Launch code-server if the ENABLE_CODE_SERVER flag is true.
if [ "$ENABLE_CODE_SERVER" = "true" ]; then
    if [ "$USE_HTTPS" = "true" ]; then
        code-server --bind-addr 0.0.0.0:$PORT --cert /etc/ssl/certs/dev-aphelion-localhost.crt --cert-key /etc/ssl/private/dev-aphelion-localhost.key --user-data-dir $CUSTOM_RC_FILE --extensions-dir $CUSTOM_NU_FILE || handle_exception "Failed to launch code-server over HTTPS"
        xdg-open "https://$LOCALHOST_URL" || handle_exception "Failed to open web browser"
    else
        code-server --bind-addr 0.0.0.0:$PORT --user-data-dir $CUSTOM_RC_FILE --extensions-dir $CUSTOM_NU_FILE || handle_exception "Failed to launch code-server over HTTP"
        xdg-open "http://$LOCALHOST_URL" || handle_exception "Failed to open web browser"
    fi
fi

# Run the application if the RUN_APPLICATION flag is true
if [ "$RUN_APPLICATION" = "true" ]; then
    ./target/release/$APP_NAME || handle_exception "Failed to run $APP_NAME"
fi

# Set up port forwarding for HTTP (80) and HTTPS (443) to the chosen port.
iptables -t nat -A PREROUTING -p tcp --dport 80 -j REDIRECT --to-port $PORT || handle_exception "Failed to set up port forwarding for port 80"
iptables -t nat -A PREROUTING -p tcp --dport 443 -j REDIRECT --to-port $PORT || handle_exception "Failed to set up port forwarding for port 443"
