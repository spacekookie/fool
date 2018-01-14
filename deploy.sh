#!/bin/bash
# This gets the version from the Cargo.toml
echo $(cat Cargo.toml | grep "version = " | awk -F\" '{print $2}' -)