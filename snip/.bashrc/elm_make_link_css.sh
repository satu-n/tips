#!/bin/bash

. ${here}/valid_num_args.sh

function elm_make_link_css() {
  valid_num_args $# 0
  local _src_file="src/Main.elm"
  local _css_file="style.css"
  local _css_pattern="<style>body { padding: 0; margin: 0; }</style>"
  local _css_replacement="<link rel=\"stylesheet\" type=\"text/css\" href=\"$_css_file\">"
  elm make $_src_file
  sed -i "s#$_css_pattern#$_css_replacement#" index.html
  return 0
}
