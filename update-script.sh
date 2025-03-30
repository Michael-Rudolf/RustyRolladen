#!/bin/bash
git pull
sudo cargo clean
sudo cargo build
sudo mv target/debug/RustyRolladen /bin