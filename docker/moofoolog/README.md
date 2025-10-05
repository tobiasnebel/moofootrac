# notes

--> in project root (not here!), run:

```
$ docker build -t moofoolog:linux .
$ docker run --rm moofoolog:linux
```

copy binary from image:
```
$ docker run -d --name my_bin_source_container moofoolog:linux
$ docker cp my_bin_source_container:/usr/local/bin/moofoolog moofoolog.linux
```