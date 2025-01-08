# Validator for devfile

The devfile validation is based on what's done in [che-devfile-registry](https://github.com/devfile/devworkspace-generator/tree/main/src/devfile-schema)

## Generate the Type from the schema

To do so we use the [typify](https://github.com/oxidecomputer/typify/tree/main) package that can be installed with:

```bash
cargo install cargo-typify
```

Then we can generate the type with:

```bash
cargo typify devfile-schema.json --output src/devfile.rs
```

To update the schema, you can use the following command:

```bash
cargo typify devfile.V230.json --output devfile_v230.rs
cargo typify devfile.V222.json --output devfile_v222.rs
cargo typify devfile.V221.json --output devfile_v221.rs
cargo typify devfile.V220.json --output devfile_v220.rs
cargo typify devfile.V210.json --output devfile_v210.rs
cargo typify devfile.V200.json --output devfile_v200.rs
```
