function set_up_bfjvm {
  export BFJVM_CONFIG_PATH=$(bf-j-vm.sh get config-path)

  random_uuid=$(bf-j-vm.sh utility rand-uuid)

  export BFJVM_CURRENT_PROMPT_ID="${random_uuid}"
  export BFJVM_CURRENT_PROMPT_PATH="${BFJVM_CONFIG_PATH}/.temp_${random_uuid}"
  export BFJVM_CURRENT_JAVA_DIR="${BFJVM_CURRENT_PROMPT_PATH}/java"
  export BFJVM_CURRENT_JAVA_HOME="${BFJVM_CURRENT_JAVA_DIR}/home"
  export BFJVM_CURRENT_JAVA_BIN="${BFJVM_CURRENT_JAVA_HOME}/bin"

  mkdir "${BFJVM_CURRENT_PROMPT_PATH}"
  mkdir -p "${BFJVM_CURRENT_JAVA_DIR}"

  java_home_to_ln=$(bf-j-vm.sh utility win-to-cyg-path $JAVA_HOME)

  ln -sfn "${java_home_to_ln}/" "${BFJVM_CURRENT_JAVA_HOME}"

  export JAVA_HOME=${BFJVM_CURRENT_JAVA_HOME}
  export PATH="${BFJVM_CURRENT_JAVA_BIN}:${PATH}"

  # deletes the temp directory
  function cleanup {      
    rm -rf "$BFJVM_CURRENT_PROMPT_PATH"
  }

  # register the cleanup function to be called on the EXIT signal
  trap cleanup EXIT
}

[[ -z "${BFJVM_CURRENT_PROMPT_ID}" ]] && set_up_bfjvm

