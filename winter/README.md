# winter

This component is the runtime enforcement, using SSH and kubernetes API to connect to a set of targets defined in `/etc/winter.toml`.

This daemon can be run within a cluster as well of course, but can also be run manually or managed by systemd, etc etc.

### kube-rs

The kube-rs crate provides a Kubernetes client library.

### libssh2 bindings

Winter uses SSH client bindings from https://libssh2.org/ (same as _cone_ uses)

## The reaction scripts

Winter can run using default values for CI/CD purposes, or can be adapted for other uses. There are several files required to compile winter that are provided using default values of "TEMPLATE", port 22 for ssh, and port 6443 for k3s. The values for the address and hostname are coded to localhost by default. Adjusting the configuration can be used to orchestrate remote systems. Changing ports to custom ports or arbitrary addresses works fine.

### Scaling winter

This daemon is designed to handle one new server and one current/old server at a time. A single winter instance can be adapted to controlling more new, current, old, or whatever servers by updating the config struct and then applying the functions to them appropriately in the main.rs logic.
