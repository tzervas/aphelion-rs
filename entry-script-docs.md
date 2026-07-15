
# Entry Script for aphelion-dev Docker Development Environment for Rust

The entry script serves as the entry point for the Docker container optimized for Rust development. This script performs several important tasks to set up and optimize the development environment.

## Features

1. **Dynamic Application Name**: The script dynamically sets the application name based on the `APP_NAME` environment variable.
2. **Run Application**: The script optionally runs the application specified by `APP_NAME` if `RUN_APPLICATION` is set to `true`.
3. **SSL Certificate Generation**: The script can generate an SSL certificate specifically for `dev.aphelion.localhost`.
4. **Port Availability Check**: It dynamically checks for port availability and configures port forwarding accordingly.

## Environment Variables

- `APP_NAME`: Specifies the name of the application. (Default: `aphelion-rs`)
- `RUN_APPLICATION`: Whether to run the application specified by `APP_NAME`. (Default: `false`)
- `ENABLE_CODE_SERVER`: Enables the code-server interface. (Default: `false`)
- `USE_HTTPS`: Enables HTTPS for code-server. (Default: `false`)
- `UPDATE_CERT`: Enables updating/replacing the SSL certificate. (Default: `false`)
- `CUSTOM_RC_FILE`: Path to a custom shell RC file. (Default: empty)
- `CUSTOM_NU_FILE`: Path to a custom `nu` shell file. (Default: empty)

## How to Run the Docker Container

You can run the Docker container with the default settings as follows:

```bash
docker run -it --name my-dev-container my-dev-image
```

To override the default settings, you can specify environment variables:

```bash
docker run -it --name my-dev-container -e APP_NAME=my_app -e RUN_APPLICATION=true my-dev-image
```

### Using Docker Compose

You can also use Docker Compose to manage your container. A sample `docker-compose.yml` file might look like:

```yaml
version: '3'
services:
  my-dev-container:
    image: my-dev-image
    environment:
      - APP_NAME=my_app
      - RUN_APPLICATION=true
```

Run it with:

```bash
docker-compose up
```

## Examples

### Running a Custom Application

If you want to run a custom application named `my_app`:

```bash
docker run -it --name my-dev-container -e APP_NAME=my_app -e RUN_APPLICATION=true my-dev-image
```

### Disabling Application Run

If you don't want to run any application:

```bash
docker run -it --name my-dev-container -e RUN_APPLICATION=false my-dev-image
```

### Enabling HTTPS and SSL Certificate

To enable HTTPS and generate an SSL certificate:

```bash
docker run -it --name my-dev-container -e USE_HTTPS=true -e UPDATE_CERT=true my-dev-image
```

### Using Custom Shell Configuration

To use a custom `.bashrc` file:

```bash
docker run -it --name my-dev-container -e CUSTOM_RC_FILE=/path/to/.bashrc my-dev-image
```

To use a custom `nu` shell file:

```bash
docker run -it --name my-dev-container -e CUSTOM_NU_FILE=/path/to/nu-file my-dev-image
```
