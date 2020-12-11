#!/bin/bash

. ~/docs/tips/snip/valid_num_args.sh

function  docker_images_filter_by_label_ref() {
  valid_num_args $# 1
  docker images -f label=ref=$1
  return 0
}
