#!/bin/bash
git pull
sudo cargo clean
sudo cargo build
rm /bin/RustyRolladen
sudo mv target/debug/RustyRolladen /bin