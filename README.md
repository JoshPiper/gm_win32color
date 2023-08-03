# gm_win32color

Minimal Reproducible Example of 24 bit color support in Garry's Mod SRCDS (main branch, Windows 10, conhost, virtual terminal enabled, using ansi colour codes)

> [!WARNING]
> This is a minimum environment, and all functions are not stable, may break, and may crash games.
> Do not use this in a production environment.

## API Reference

### `win32color.test(): void`
Prints out 3 colour bars, red, green and blue, the width of the console window.
Left side is black, right side is full saturation. 
  
### `win32color.truecolor_print(red, green, blue, string): void`
Prints a string to console with the given colour.

### `win32color.get_console_size(): (int, int)`
Gets the width and height of the console window (columns / rows)
