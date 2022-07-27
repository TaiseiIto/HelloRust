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

