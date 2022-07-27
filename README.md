# HelloRust

I learn Rust with reference to [this page](https://doc.rust-jp.rs/book-ja/) in this repository.

## Docker environment

### Build a docker environment and log in to it.

```
$ make devenv
...
~/HelloRust # 
```

### Rebuild the environment and log in to it.

```
~/HelloRust # exit
$ make rebuild-devenv
...
~/HelloRust # 
```

### Delete the environment.

```
~/HelloRust # exit
$ make clean-devenv
$
```

## In the docker environment

### Build the Rust projects

There is a `projects` directory that has Rust projects.

```
~/HelloRust # make
...
~/HelloRust #
```

### Create a new project

Create a new project `new_project`.

```
~/HelloRust/projects # make project NAME=new_project
...
~/HelloRust/projects #
```

## For the developper

### git config

Only the developer can execute it.

```
$ make gitconfig KEY=<GitHub private key path> GPG=<.gnupg path>
```

