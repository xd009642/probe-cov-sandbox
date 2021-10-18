# probe-cov-sandbox

## Intro

So I don't necessarily have the hardware you'll have so I'll do it with a
random STM32 board I have and show my steps to work out how to piece it all
together.

## Setting up your system

So for test I've used an STM32L476RG Nucleo board which uses ARMv7E-M with a
hardware FPU. This uses the rust target thumbv7em-none-eabihf. So to install
this target:

```
rustup target add thumbv7em-none-eabihf
```

We can break up the target triple as `<arch>-OS-eabi<fpu>`. EABI is Embedded
Application Binary Interface, we don't have to care about that. Also as we're
working bare metal we put none for the OS. We have a hardware FPU so hf is added
otherwise it's an empty string. And the first part is the ARM ISA.

For your own board look up the part number of the STM32 chip on the board, find
the arm architecture and if the chip has a hard FPU. Then we can fill in the
triple accordingly.

We can then install some commonly used tools:

```
# Taken from cargo-flash prerequistes for linux. Check resources
sudo apt-get install pkg-config libusb-1.0-0-dev libftdi1-dev -y
cargo install cargo-binutils
rustup component add llvm-tools-preview
cargo install cargo-flash
cargo install flip-link
cargo install cargo-embed
```

## Programming the board

Prerequisite - plugging in the board, making sure some LEDs come on.

Now we know our chip is the STM32L467RG. If we look at the datasheet we can see
more specifically it's the `STM32L476RGT6U` now part of that is the actual ISA
and hardware details, some stuff at the end probably refers to the size, pin
layout and dimensions of the chip and don't affect programming it just the
hardware design. 

Before we can do anything we need to sort out the memory map for the linker,
which is gonna be the most annoying part. I've added memory.x.template in the
test-project for the template one in the cortex-m-quickstart. And this is how I
figure it out for my chip which is filled in the memory.x included in the
project.

Firstly we'll go to the datasheet for the chip (google it, go to stm page and
click download datasheet). Then we go to section 5 called "Memory Mapping". For
other chips it might be in a different place but the name is fairly standard.

Now the memory map looks like this:

![Two columns showing high level and low level regions of memory in the MCU
in the high level we see SRAM1 stars at 0x20000000 and doesn't have a determined
end. In the low level we ee flash starts at 0x08000000 and ends at 0x08100000](images/stm32l476rg_memmap.png)

So we're going to pick the region called Flash memory for the flash and the
origin is `0x0800_0000` and it ends at `0x0810_0000` so I'll set the flash
origin address to 0x08000000 and the length to 1048K since it seems I have a
larger flash than the example chip. Also I can see SRAM1 starts at 
`0x2000_0000` and there are 2 SRAM blocks. So we can't map all of the
128KB mentioned on the chips website to this RAM origin as this memory probably
exists in both blocks. So lets Ctrl+F for SRAM1 and in section 3.5 it says there is
96KB of SRAM at `0x2000_0000`. So using that I fill in the flash origin and length
and the RAM origin and length.

Now we can find out cargo-flash target using the first part of the name in the
board name.

```
cargo flash --list-chips | grep STM32L476RG
```

And we get the output: `STM32L476RGTx`, x in hardware language is "don't care"
so this is probably the pin layout details mentioned before. Now if we were
pre-probe-rs I'd go over install openocd or setting up a gdb remote instance
connected to the board. But now life is easier than this, and we can use 
cargo-embed to program the device and set up semihosting so we can see the
printout. 

If we didn't use cargo-embed and didn't set up semihosting when we flash the
device it would look like nothing happened! But instead the Embed.toml has our
config info and we can just run:

```
cargo-embed --example simple
```

And now we should see our printout. *TODO we don't, maybe we do need to sort
out semihosting ourselves?*

TODO Fill in semihosting things.

```
cargo flash --release --chip STM32L476RGTx --example simple
```

## Resources

* https://github.com/probe-rs/cargo-flash#prerequisites
* https://github.com/rust-embedded/cortex-m-quickstart 
* https://github.com/knurling-rs/flip-link
* https://ferrous-systems.com/blog/defmt-test-hal/ 
* https://ferrous-systems.com/blog/test-embedded-app/
* https://probe.rs/docs/library/basics/
