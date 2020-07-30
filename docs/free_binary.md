# Build for a free binary

1. cargo build --target thumbv7em-none-eabihf

# Alternative builds for current os

### Linux
1. cargo rustc -- -C link-arg=-nostartfiles
### Windows
2. cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
### macOS
3. cargo rustc -- -C link-args="-e __start -static -nostartfiles"