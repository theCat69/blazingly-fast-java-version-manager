#!/bin/bash

# Copyright (C) 2023 Félix Vadcard
# see main.rs for details

args=("$@")
args=("--shell" "git_bash" "${args[@]}")

export BFJVM_SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

$BFJVM_SCRIPTPATH/deps/bf-j-vm.exe "${args[@]}"
