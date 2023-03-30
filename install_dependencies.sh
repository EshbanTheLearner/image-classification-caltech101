#!/bin/bash

cd ~/

wget https://github.com/intel/mkl-dnn/releases/download/v0.19/mklml_mac_2019.0.5.20190502.tgz

gunzip -c mklml_mac_2019.0.5.20190502.tgz | tar xvf -

wget https://download.pytorch.org/libtorch/cpu/libtorch-macos-1.1.0.zip

unzip libtorch-macos-1.1.0.zip

export LD_LIBRARY_PATH=mklml_mac_2019.0.5.20190502/lib:"$LD_LIBRARY_PATH"
export LIBTORCH=libtorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH