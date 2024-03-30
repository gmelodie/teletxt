# TODO
- [x] Logging (production vs dev)

- [x] Docker
    - [x] Create Docker container
    - [x] Multi stage build for docker (smaller images)
    - [x] Use volume to hold data

- [x] Make bot run forever (websocket, for running inside docker)
    - [x] Load file, do not throw anything out
        - [ ] implement Merge for todo
    - [x] Give me the file stored in server (export)

- [x] Add Commands
    - [x] /start
    - [x] /download
    - [x] /upload

- [x] Give user feedback

- [x] UX
    - [x] Give user more information about errors
    - [x] Tell user where in todo file syntax error is (which day/section)

- [ ] Remove done section

- [ ] New data structure for Days (sorted, hashmap)

- [ ] v0.2.0? record of what's done instead of todo

- [x] move Result and err macro to util.rs, import on others
