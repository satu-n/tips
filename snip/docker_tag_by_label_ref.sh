#!/bin/bash

. ~/docs/tips/snip/valid_num_args.sh

function docker_tag_by_label_ref() {
  valid_num_args $# 1
  docker images -f label=ref=$1 -q \
  | head -n 1 \
  | xargs -I {} docker tag {} $1
  return 0
}
