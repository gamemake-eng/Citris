#   Plaster FX spec

Plaster FX is a 2d graphics engine designed in-house by Encon consumer systems. The graphics engine is used in the gpu that powers the CITRIS PORTABLE CONSOLE.

##  Capabilities

### The 2 buffers

The Plaster FX engine has the ability to draw 2d graphics to one of it's 2 buffers, the back and front buffer. The back buffer is the "Working" buffer that we draw to while the front buffer is the "Visible" one that we copy too that is shown to the screen.

### Drawing

The Plaster FX engine allows programmers to either blit a section of the 1024x1024 texture buffer to the back buffer or fill the backbuffer with a color.

### Shaders

The Plaster FX can directly interface with the PixComp shader core allowing it to apply shaders to either it's back buffer or texture buffer.

##  Interface

The Plaster FX interfaces with the cpu with commands and registers. Commands tell the gpu what to do while the registers act as parameters for those commands.

There are 3 commands and 7 registers that can be used by developers. They are listed bellow.

### Registers

DX, DY -> The destination of the texture being blitted

SX, SY -> The position of the section of the texture you want to blit

W, H -> The size of the texture section you want to blit

CC -> The color in RGB format that you want to clear the screen with.

### Commands

BLIT -> Blits the section of the texture specified by SX, SY, W, and H to a position on the back buffer specified by DX and DY

CLS -> Clears the back buffer with the color specified by CC

FLIP -> Copies the back buffer to the front buffer
