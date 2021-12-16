# pp 

A very basic pretty printer, pipe your json log output into it and it will make them more readable: 

```shell
echo '{"label": "my-log", "level":"debug", "message": "hi"}' | pp
```

Outputs:

<pre>[<span style="color: blue">debug</span>] <span style="color: violet">my-log</span> - hi</pre>


## logging stderr

You'll need to send stderr into stdout: 

```shell
your_log_command 2>&1 | pp
```


## run demo

```shell
./demo.sh | cargo run
```


