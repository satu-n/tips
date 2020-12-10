#!/bin/bash

function docker_run_interactive() {
  if [ $# -ne 1 ]; then
    {
      echo "$# arguments found."
      echo "Specify exactly 1 image."
    } 1>&2
    return 1
  else
    local docker_ps1='${debian_chroot:+($debian_chroot)}🐳 \[\e[1;36m\]\h\[\e[m\]:\[\e[1;34m\]\W\[\e[m\]\$ '
    docker run --rm -ite "DOCKER_PS1='$docker_ps1'" $1 bash -c 'echo "PS1=$DOCKER_PS1" >>~/.bashrc && bash -l'
    return 0
  fi
}
