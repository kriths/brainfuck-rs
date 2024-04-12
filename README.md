Yet another implementation of a brainfuck interpreter, written in Rust.

<small>
No practical reason for this - only to practice some rust and learn brainfuck syntax... :^)
</small>

## Running

Run a brainfuck file by calling as the argument to this program:

```sh
> cargo run -- examples/hello-world.bf
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/brainfuck-rs examples/hello-world.bf`
Hello World!
```

## Examples

Simple examples with a permissible source can be found in the `examples` directory.
They have been copied from wikipedia (CC-BY-SA) at their commit times.

## References

- Wikipedia: https://en.wikipedia.org/wiki/Brainfuck
- Esolangs: https://esolangs.org/wiki/Brainfuck
- Examples: https://www.brainfuck.org/
