To run the project, run this command in the root:

cargo run

To install a crate that you want to import at the top of a file, run this command to
add it to your Cargo.toml file:

cargo add crate-name

How this affects Cargo.lock is that it will add the rand crate and its dependencies to the lock file,
ensuring that the same versions are used when building the project in the future.
Note that the lock file is automatically generated and should not be edited manually.

To update the crate later, run:

cargo update -p crate-name

This will update the crate to the latest version allowed by the version specified in Cargo.toml.
To update to a specific version, run:

cargo add crate-name@0.9.0

Replacing 0.9.0 with the desired version. This will update the version in Cargo.toml and update the crate in cargo.lock accordingly.
