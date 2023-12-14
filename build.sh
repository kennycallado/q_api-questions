#!/bin/bash

# Dependencies:
# - cargo-zigbuild
# - zig

# This script compiles the binary using the rust-musl-builder docker image,
# and build the docker images for multiples platforms.
#
# In order to make the images to work, you need to install
# qemu-user-static in the host machine, because the binary
# is compiled for a different architecture.
#
# Introduce the option to build the stable version. To do so, run:
# ./build.sh stable
set -e

platforms=("linux/amd64" "linux/arm64")

# Get the package name and version from Cargo.toml
package_name=$(cat Cargo.toml | grep 'name' | awk '{print $3}' | tr -d '"')
version=$(cat Cargo.toml | grep 'version' | head -1 | awk '{print $3}' | tr -d '"')
database=$(cat Cargo.toml | grep '^default' | awk '{print $3}' | grep 'db' | tr -d '",[]' )

# compile_zigbuild() {
#   cargo-zigbuild build --release --target $target
# }

compile_muslrust() {
  podman run --rm -it \
    -v "$(pwd)":/volume clux/muslrust:stable \
    cargo build --release 
}

# Remove Cargo.lock
# rm -f Cargo.lock

# Permissions for target folder
mkdir -p target
chmod -R o+w target

# Build the binary
if [ "$database" == "db" ]; then
  compile_muslrust
fi

for platform in ${platforms[@]}; do
  echo "Building docker image for: $platform."

  # get the tag
  tag=$(echo "${platform//\//_}" | tr -d 'linux_' | xargs -I {} echo {})
  target="x86_64-unknown-linux-musl"

  if [[ $platform == *"arm"* && "$database" != "db_diesel" ]]; then
    target="aarch64-unknown-linux-musl"
  fi

  # Build the binary
  # if [ "$database" != "db_diesel" ]; then
  #   compile_zigbuild
  # fi

  # build the image
  podman build --no-cache --pull \
    --platform ${platform} \
    -t kennycallado/${package_name}:${version}-${tag} \
    --build-arg PACKAGE_NAME=${package_name} \
    --build-arg TARGET=${target} \
    -f ./Containerfile .

  # push the images
  podman push kennycallado/${package_name}:${version}-${tag}
done

# create version manifest
podman manifest create kennycallado/${package_name}:${version}
for platform in ${platforms[@]}; do
  tag=$(echo "${platform//\//_}" | tr -d 'linux_' | xargs -I {} echo {})
  podman manifest add --arch ${tag} kennycallado/${package_name}:${version} kennycallado/${package_name}:${version}-${tag}
done

# create manifest latest manifest
podman manifest create kennycallado/${package_name}:latest
for platform in ${platforms[@]}; do
  tag=$(echo "${platform//\//_}" | tr -d 'linux_' | xargs -I {} echo {})
  podman manifest add --arch ${tag} kennycallado/${package_name}:latest kennycallado/${package_name}:${version}-${tag}
done

# create manifest stable manifest
if [ "$1" == "stable" ]; then
  podman manifest create kennycallado/${package_name}:stable
  for platform in ${platforms[@]}; do
    tag=$(echo "${platform//\//_}" | tr -d 'linux_' | xargs -I {} echo {})
    podman manifest add --arch ${tag} kennycallado/${package_name}:stable kennycallado/${package_name}:${version}-${tag}
  done

  # push the stable manifest
  podman manifest push --rm kennycallado/${package_name}:stable docker://kennycallado/${package_name}:${version}
fi

# push the rest of the manifests
podman manifest push --rm kennycallado/${package_name}:${version} docker://kennycallado/${package_name}:${version}
podman manifest push --rm kennycallado/${package_name}:latest docker://kennycallado/${package_name}:${version}

# remove the images
for platform in ${platforms[@]}; do
  tag=$(echo "${platform//\//_}" | tr -d 'linux_' | xargs -I {} echo {})
  podman rmi kennycallado/${package_name}:${version}-${tag}
done

# remove the manifest
podman system prune -f

exit 0
