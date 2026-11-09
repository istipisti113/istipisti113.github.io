# My setup

## Laptop 
- Asus Tuf gaming f15
- Additional m.2 SSD, 2TB hard drive and double the memory. 
- Intel(R) Core(TM) i5-10300H CPU @ 2.50GHz 4 core 8 thread
- 16GB DDR4
- Nvidia 1650 mobile
## Desktop 
- Intel(R) Core(TM) i7-4790S CPU @ 3.20GHz 4 core 8 thread
- 16 GB DDR4 
- No video card (it broke ages ago)
- minecraft server mainly

## OS  and dotfiles
Both machines run NixOS, their configuration is managed in one file (flake), giving one source of truth for them. Configuration and some packages are shared between them.

My dotfiles are managed with Nix too, with Home Manager, tracked with git along with the configuration. If I install a new neovim plugin or change something, It will have the same effect on both machines. 

I use sway on the laptop and KDE Plasma on the desktop, tho  it is mostly headless.

## Peripherials and other goodies

![setup image](https://github.com/tompi/cheapino/raw/master/doc/gallery/20.jpg)
I no longer use arch (btw), but the setup remains mostly unchanged.

The keyboard is a Cheapino V2, with a 3d printed case on my 3d printer: Anycubic kobra 2 neo, managed with a Raspberry Pi 4