#!/bin/bash
# build.sh
# Script to build the smart contracts and frontend application

set -e

echo "Initializing Anchor workspace..."
anchor init svmivacy || true

echo "Building smart contracts..."
cd svmivacy
anchor build

echo "Running tests..."
anchor test

echo "Building frontend application..."
cd frontend
npm install
npm run build
cd ..