# tips

- [tips](#tips)
  - [git branch at bash prompt](#git-branch-at-bash-prompt)
  - [tagging docker compose intermediate images](#tagging-docker-compose-intermediate-images)
  - [docker command aliases](#docker-command-aliases)
  - [bash prompt in docker container](#bash-prompt-in-docker-container)
    - [Solution:](#solution)

## git branch at bash prompt

To show git branch at bash prompt,

Add like `$(__git_ps1 ">%s")` to `PS1` of `~/.bashrc`:

![git_branch_at_bash_prompt](images/git_branch_at_bash_prompt.png)

## tagging docker compose intermediate images

To name each intermediate image of a service of a Compose project,

first name each build stage `AS {stage}` in `{project}/{service}/Dockerfile`:

```Dockerfile
FROM image AS stage
```

Then in `{project}/{service}` dir, build and tag the image targeting `{stage}`.

```bash
docker build . -t $(basename $(dirname $(pwd)))_$(basename $(pwd)):{stage} --target {stage}
```

The image will be tagged as `{project}_{service}:{stage}`.

![tagging_docker_compose_intermediate_images](images/tagging_docker_compose_intermediate_images.png)

<!-- ## Thank you for reading! -->

## docker command aliases

[Here](snip/.bash_aliases/docker.sh)
is my docker aliases.

Don't forget to add to `~/.bash_aliases` as follows:

```bash
readonly BASH_ALIASES_DIR="**/tips/snip/.bash_aliases"
. ${BASH_ALIASES_DIR}/.hub.sh
```

For `# myfunc`, see the script of the same name [here](snip/.bashrc)

If you want to use your own function like this, don't forget to add to `~/.bashrc` as follows:

```bash
readonly BASHRC_EXTENSION_DIR="**/tips/snip/.bashrc"
. ${BASHRC_EXTENSION_DIR}/.hub.sh
```

## bash prompt in docker container

Here is 2 way to interact with a running container:

* `docker run -it IMAGE COMMAND`
* `docker exec -it CONTAINER COMMAND`

First, run salmon container of rice image. Then...

![holy_shit_ruin](images/holy_shit_ruin.png)

Holy shit, what a surprising white prompt!

This prompt is the `PS1` itself of `CONTAINER:~/.bashrc`

No matter how many times it is overwritten, it will return to white next time.

Bye, my ephemeral `PS1`...

### Solution:

Add to `CONTAINER:~/.bashrc` and restart `bash`

```bash
docker run -ite "DOCKER_PS1='$docker_ps1'" {IMAGE} bash -c 'echo "PS1=$DOCKER_PS1" >>~/.bashrc && bash -l'
```

[Here](snip/.bashrc/docker_run_interactive.sh) is the code I'm actually using, which gives the following result:

![bash_prompt_in_docker_container](images/bash_prompt_in_docker_container.png)

Hello, my pseudo-persistent `PS1`!

Let's look at the container counting down at `run -it` (left) from `exec -it` (right).

![docker_run_exec_interactive](images/docker_run_exec_interactive.gif)

<!-- Thank you for reading! -->
__Thank you for reading!__
