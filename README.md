# QRcade

QRcade is an experimental game engine that enables games to be distributed and played through QR codes. By storing compiled Lua bytecode in QR codes, games can be instantly loaded and played by simply scanning the code. This unique approach opens up interesting possibilities for game distribution and sharing.

## 🎮 Key Features

### Core Engine
- Entity Component System (ECS) architecture for efficient game object management
- Lua scripting integration for game logic
- Performant 2D rendering using SDL2
- Robust event system for input handling

### Game Development Tools
- Text rendering system for UI and game messages
- Tilemap system for level design
- Physics system with gravity and collision detection
- Drag-and-drop interaction system
- Debug visualization tools for development

### Component Systems
- Transform system for positioning and rotation
- Velocity system for movement
- Collision system for object interaction
- Renderable system for sprites and shapes
- Text system for UI elements
- Tilemap system for level design

## 🚧 Project Status

QRcade is currently in early alpha development. While the core systems are functional, they are still being refined and may undergo significant changes. The project is actively being developed with a focus on:

- Stability and performance improvements
- Enhanced documentation and examples
- Additional game development features
- Better error handling and debugging tools
- QR code generation and scanning optimization

### Known Limitations
- Limited to 2D games currently
- Basic physics implementation
- QR code size limitations affect game complexity
- Work in progress documentation

## 📖 Documentation

The engine includes comprehensive Lua API documentation for game development:
- [Engine Documentation](engine/docs/index.md)
- [API Reference](engine/docs/index.md#documentation-overview)
- [Example Games](engine/resources/lua_scripts/)

## 🛠️ Building from Source

### Prerequisites
- Rust (latest stable)
- SDL2 development libraries
- Lua 5.4

### Build Instructions
```bash
# Install SDL2 development libraries
# Ubuntu/Debian:
sudo apt-get install libsdl2-dev

# macOS:
brew install sdl2

# Build the project
cargo build --release
```

## 🎮 Example Games

The `resources/lua_scripts` directory contains several example games showcasing different engine features:
- Snake game implementation
- Asteroids clone
- 2048 puzzle game
- Physics simulation demo
- Drag and drop examples

## 🤝 Contributing

While the project is in early development, contributions and feedback are welcome. Please feel free to:
- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

## 📝 License

This project is licensed under Creative Commons - see the [LICENSE](LICENSE) file for details.

## 📫 Contact

Feel free to open GitHub issues for:
- Bug reports
- Feature requests
- Documentation improvements
- General questions

## ⭐ Acknowledgments

Special thanks to:
- SDL2 development team
- Lua development team
- All contributors and testers

---

**Note:** This is an experimental project in early alpha development. Features and APIs may change significantly between versions.
