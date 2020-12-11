#!/bin/bash

function valid_num_args() {
  if [ $# -ne 2 ]; then
    echo 'usage: valid_num_args $# {expected}' 1>&2
    return 1
  fi
  if [ $1 -ne $2 ]; then
    {
      echo "$1 arguments found."
      echo "Specify exactly $2 arguments."
    } 1>&2
    return 2
  fi
  return 0
}
