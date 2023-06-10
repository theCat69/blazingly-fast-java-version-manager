#!/bin/bash

HELP=""
VERSION=""
LOCAL=""
args=("$@")

while [[ $# -gt 0 ]]; do
  case $1 in
    -h|--help)
      HELP="YES"
      shift 
      ;;
    s|switch)
      VERSION="$2"
      shift 
      shift
      ;;
    -l|--local)
      LOCAL="YES"
      shift
      ;;
    *) 
      shift
      ;;
  esac
done

args=("--shell" "git_bash" "${args[@]}")

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

$SCRIPTPATH/deps/bf-j-vm.exe "${args[@]}"
