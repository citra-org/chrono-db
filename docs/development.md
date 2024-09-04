# Development Docs


Start chrono db with live changes
```
cargo watch -w src -x 'run start <chrono>'
```

Sample conf.ini
```
encoding=utf-8

[CHRONO_DB]
; main configurations
HOST=127.0.0.1
PORT=3141

[STREAM]
; purging methods

METHOD=TIME
VALUE=2

; METHOD=STORAGE
; VALUE=5
```
---
### Sample commands

INSERT INTO stream VALUES ('header1', 'body1'), ('header2', 'body2'), ('header3', 'body3')

INSERT INTO stream VALUES ('header1', 'body1')