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

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

if [ ! -z "$HELP" ]; then
  $SCRIPTPATH/deps/bf-j-vm.exe "${args[@]}"
elif [[ ! -z "$VERSION" && ! -z "$LOCAL" ]]; then
  $SCRIPTPATH/deps/bf-j-vm.exe -h 
  echo Changing local version on git bash is not supported
elif [[ ! -z "$VERSION" ]]; then
  args+=('--shell')
  args+=('git_bash')
  $SCRIPTPATH/deps/bf-j-vm.exe "${args[@]}"
else
  $SCRIPTPATH/deps/bf-j-vm.exe "${args[@]}"
fi
