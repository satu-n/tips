# tips

- [tips](#tips)
  - [git branch at bash prompt](#git-branch-at-bash-prompt)
  - [tagging docker intermediate images](#tagging-docker-intermediate-images)

## git branch at bash prompt

To show git branch at bash prompt,

Add like `$(__git_ps1 ">%s")` to `PS1` of `~/.bashrc`:

![git_branch_at_bash_prompt](images/git_branch_at_bash_prompt.png)

## tagging docker intermediate images

To name each build stage as a tagged image,

first label it in `Dockerfile`:

```Dockerfile
FROM image AS tag
LABEL ref="project_service:tag"
```

Then filter images by the label and run `docker tag`:

```bash
REF="project_service:tag" \
docker images -f label=ref=$REF -q \
| head -n 1
| xargs -I {} docker tag {} $REF
```

All of the above looks like this:

![docker_build_stage_later_tagging](images/tagging_docker_intermediate_images.png)

<!-- ## Thank you for reading! -->

__Thank you for reading!__
