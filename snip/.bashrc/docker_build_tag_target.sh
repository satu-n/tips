#!/bin/bash

. ${here}/valid_num_args.sh

function docker_build_tag_target() {
  valid_num_args $# 1
  docker build . -qt $(basename $(dirname $(pwd)))_$(basename $(pwd)):$1 --target $1
  return 0
}
