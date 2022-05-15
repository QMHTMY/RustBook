#!/bin/bash

# Linux 下安装 Rust

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

echo " " >> ~/.bashrc
echo "# Rust 语言环境变量" >> ~/.bashrc
echo "export RUSTPATH=$HOME/.cargo/bin" >> ~/.bashrc
echo "export PATH=$PATH:$RUSTPATH" >> ~/.bashrc
echo " " >> ~/.bashrc

source ~/.bashrc
