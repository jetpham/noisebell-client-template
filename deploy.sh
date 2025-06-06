#!/bin/bash

# Exit on error
set -e

echo "Building for Raspberry Pi..."
cross build --release --target aarch64-unknown-linux-gnu

echo "Copying to Raspberry Pi..."
scp target/aarch64-unknown-linux-gnu/release/noisebell-client-template noisebridge@noisebell.local:~/noisebell-client-template

echo "Setting permissions"
ssh noisebridge@noisebell.local "chmod +x ~/noisebell-client-template/noisebell-client-template"

echo "Deployment complete!" 
