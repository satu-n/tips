#!/bin/bash

. ${here}/valid_num_args.sh

function fn() {
  valid_num_args $# 1
  #
  return 0
}
