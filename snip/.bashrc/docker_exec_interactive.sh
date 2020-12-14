#!/bin/bash

. ${here}/valid_num_args.sh
. ${here}/docker_ps1.sh

function docker_exec_interactive() {
  valid_num_args $# 1
  docker exec -ite "DOCKER_PS1='$docker_ps1'" $1 bash -c 'echo "PS1=$DOCKER_PS1" >>~/.bashrc && bash -l'
  return 0
}
