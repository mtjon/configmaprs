# Configmaprs

The aim of this binary is to map environment variables to a file, akin to
Kubernetes'
[ConfigMaps](https://kubernetes.io/docs/concepts/configuration/configmap/)
feature. This can be useful in environments where configarion files need to be
injected during runtime.


# Usage

For each mapping, add two environment variables, 1) a variable ending in
`_CONFIGMAP` and containing a triple of `path`, `Permissions`, and `cfg`, and
2) a config to write to the file the key of which is the value of the `cfg`
component of the triple.

For example, in order to add a `config.yaml` to `/etc/app/config.yaml` with
ReadOnly (444) permissions, add the following variables:

```sh
TEST_CONFIGMAP=/etc/app/config.yaml,ReadOnly,TEST_CFG
TEST_CFG=<yaml>
```


# Limitations

- The use of this binary requires embedding it in the image you wish to use it
  in. 
- The binary is executed during runtime by the (Docker) user. As such, it needs
  to be part of the entrypoint of the image and config files can only be
  written to directories that the user has write access to.
- Currently uses Rust's `std::fs::permissions` module, which is not
  Unix-specific, permissions are therefore limited to ReadOnly (444) and
  ReadWrite (666).



