#!/bin/bash

alias d='docker'
alias db='docker build -q .'
alias dbt='docker_build_tag_target' # myfunc
alias dr='docker run --rm $(docker build -q .)'
alias dri='docker_run_interactive $(docker build -q .)' # myfunc
alias dri_='docker_run_interactive' # myfunc
alias dl='docker logs -f'
alias dx='docker exec'
alias dxi_='docker_exec_interactive' # myfunc

alias dc='docker compose'
alias dcb='docker compose build'
alias dcr='docker compose run --rm'
# alias dcu='docker compose up -d'
alias dcl='docker compose logs -f'
alias dcx='docker compose exec'
# alias dcd='docker compose down'

alias img='docker image'
alias ctn='docker container'
