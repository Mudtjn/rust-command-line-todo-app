# Command Line based rust to do app

A simple command line app that allows you to add, remove tasks from command line

1. To list all the pending tasks
```bash
cargo run -- list
```
2. To add tasks 
```bash
cargo run -- add "<task>"
```
3. To mark a task at position y as complete
```bash
cargo run -- done <y>
```

You can do this in a custom file by adding -j or --journal-file tag and then the fileName.  