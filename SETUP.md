# Cornerstone Project Setup Guide

This guide will walk you through setting up the Cornerstone project on your local development environment. Cornerstone is an open-source project that provides a foundation for building web applications with various features.

## Prerequisites

Before you begin, ensure you have the following prerequisites installed on your system:

- Rust (https://www.rust-lang.org/tools/install)
- Git (https://git-scm.com/downloads)

## Getting Started

Follow these steps to set up the Cornerstone project:

1. Clone the Cornerstone Repository:

```shell
   git clone https://github.com/DanielOwenRaine/cornerstone.git
   cd cornerstone
```

2. Build the project:

```shell
    cargo build
```


## Configuration

# Cornerstone uses a configuration file (config.toml) to manage feature flags. You can choose to configure the project manually or use the setup wizard.

## Manual Configuration

   # Create a config.toml file in the project root directory.

   # Edit the config.toml file to set feature flags. For example:

   # auth, database

   # auth_enabled = true
   # database_enabled = true

## Using the Setup Wizard

To run the setup wizard, use the following command:

```shell
    cargo run -- --setup-wizard
```

Follow the prompts to configure the project features.
Running the Project

Once you have configured the project, you can run it with the following command:

```shell
    cargo run
```

## Additional Notes

    The project is designed to be modular, allowing you to enable or disable specific features based on your requirements.

    Refer to the project documentation for more detailed information on project features and usage.