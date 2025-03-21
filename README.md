# Distillation Tower Monitor

An open-source project built with [Tauri](https://v2.tauri.app/) and React for monitoring a distillation tower. The application measures temperatures and calculates per-plate compositions, displaying real-time charts and saving the measurement history in Excel/CSV files.

## Features

- **Real-Time Monitoring:**
  Display interactive charts of temperatures and compositions for each plate of the tower.

- **Measurement History:**
  Save historical data and export it in Excel or CSV format.

- **Modbus RTU Connection:**
  Connect to the distillation tower via Modbus RTU with no extra drivers or configurations required.

- **Flexible Configuration:**
  Automatically generates and manages a configuration file, allowing parameter customization.

## System Requirements

- **Supported Platforms:**
  Works on Windows, macOS, and Linux.

- **Dependencies:**
  No additional drivers are required for the Modbus RTU connection.

## Installation

### Backend (Tauri)
No prior configuration is needed. To compile and run the backend, please refer to the [Tauri documentation](https://v2.tauri.app/).

### Frontend (React)
The project uses [pnpm](https://pnpm.io/) for dependency management.

1. Install dependencies:
```bash
pnpm install
```

2. Start development mode:
```bash
pnpm dev
```

## Running the Project

### Development Mode
After installing dependencies, run:
```bash
pnpm dev
```
This command compiles and launches the application in development mode, allowing you to see real-time updates.

### Production Mode
For production builds, please follow the [Tauri distribution guide](https://v2.tauri.app/).

## Project Structure

The backend is organized into several modules to separate concerns:

- **calculations:**
  Contains functions for computing compositions and temperatures.

- **data_manager:**
  Handles data management and storage of measurement history.

- **emitter:**
  Contains functions invoked from the frontend to start or stop data transmission.

- **modbus_serial:**
  Manages the Modbus RTU connection and related requests.

- **settings:**
  Manages application configurations, including loading and saving settings.

- **utils:**
  Provides helper functions for exporting data, opening the file explorer, etc.

## Contributions

This is an open-source project, and contributions, improvements, and suggestions are welcome!
If you would like to contribute, please open a pull request or an issue in the repository.

## License

The project is open-source but currently does not specify a license.
If you wish to use or contribute, please note that a formal license may be defined in the future.

## Additional Resources

- [Tauri Documentation](https://v2.tauri.app/)
- [pnpm Documentation](https://pnpm.io/)

---
Enjoy monitoring your distillation tower, and thank you for using this project!
