# winter

This component is the runtime enforcement, using SSH and kubernetes API to connect to a set of pre-defined targets.

This daemon can be run within a cluster as well of course, but can also be run manually or managed by systemd, etc etc.

### kube-rs

The kube-rs crate provides a Kubernetes client library.

### libssh2 bindings

Winter uses SSH client bindings from https://libssh2.org/ (same as _cone_ uses)
