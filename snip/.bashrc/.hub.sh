#!/bin/bash

here="$(dirname ${BASH_SOURCE})"
. ${here}/docker_run_interactive.sh
. ${here}/docker_exec_interactive.sh
. ${here}/docker_build_tag_target.sh
. ${here}/elm_make_link_css.sh
