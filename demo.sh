#!/usr/bin/env bash

printf '{ "level" : "info", "message": "no label for this"}\n'
printf '{ "message": "this message has no level", "label" : "bar"}\n'
printf '{ "label":  "foo", "level" : "info", "message": "this message has foo label  "}\n'
printf '{ "level" : "silly", "message": "hi"}\n'