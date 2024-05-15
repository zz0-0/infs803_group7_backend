Sure, here is the translated README file in English, combining the provided information and screenshots:

---

# Movie Explorer App

## Project Overview

The Movie Explorer App is a cross-platform tool for browsing and managing movies. Users can browse movie lists, add interesting movies to their favorites, and rate them. The application consists of a frontend and backend, with the frontend built using Flutter and the backend using Rust with the Axum framework. Firebase is used as the database.

## Table of Contents

- [Installation Guide](#installation-guide)
- [Quick Start](#quick-start)
- [Features](#features)
- [Tech Stack](#tech-stack)
- [How to Contribute](#how-to-contribute)
- [License](#license)
- [Contact](#contact)
- [Acknowledgments](#acknowledgments)

## Installation Guide

### Frontend Installation

1. Ensure you have the [Flutter](https://flutter.dev/docs/get-started/install) environment installed.
2. Clone the repository and navigate to the frontend project directory:

   ```bash
   git clone <your-repo-url>
   cd your_repo_directory
   ```

3. Get Flutter dependencies:

   ```bash
   flutter pub get
   ```

### Backend Installation

1. Ensure you have the [Rust](https://www.rust-lang.org/tools/install) environment installed.
2. Clone the repository and navigate to the backend project directory:

   ```bash
   git clone <your-repo-url>
   cd your_repo_directory/backend
   ```

3. Build and run the backend service:

   ```bash
   cargo build
   cargo run
   ```

## Quick Start

1. Start the backend service:

   ```bash
   cargo run
   ```

2. Start the frontend application:

   ```bash
   flutter run
   ```

3. Open your browser and access the frontend application (default at http://localhost:8080).

## Features

- Browse movie lists
- Add movies to favorites
- Rate and review movies
- User registration and login
- Manage personal information

## Tech Stack

- **Frontend**:
  - Flutter
  - Riverpod (global state management)
  - go_router (routing management)

- **Backend**:
  - Rust
  - Axum (web framework)
  - Firebase (database)

## How to Contribute

We welcome contributions of any kind! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the [MIT License](LICENSE).

## Contact

For any questions or suggestions, please contact us at:

- Email: your_email@example.com
- GitHub Issues: [issues](https://github.com/your_repo/issues)

## Acknowledgments

Thanks to the developers and contributors of the following projects and tools:

- Flutter
- Rust
- Axum
- Firebase

---

## Project Files and Screenshots

### File Structure

#### Frontend Project

```plaintext
your_repo_directory/
│
├── android/
├── assets/
├── ios/
├── lib/
├── linux/
├── macos/
├── test/
├── web/
│   ├── icons/
│   │   ├── Icon-192.png
│   │   ├── Icon-512.png
│   │   ├── Icon-maskable-192.png
│   │   ├── Icon-maskable-512.png
│   ├── favicon.png
│   ├── index.html
│   ├── manifest.json
│
├── windows/
│
├── .gitignore
├── .metadata
├── README.md
├── analysis_options.yaml
├── pubspec.yaml
```

#### Backend Project

```plaintext
your_repo_directory/backend/
│
├── src/
│   ├── favorite.rs
│   ├── jwt.rs
│   ├── main.rs
│   ├── movie.rs
│   ├── user.rs
│
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── MOCK_DATA(1).json
├── MOCK_DATA(2).json
├── MOCK_DATA copy.json
├── MOCK_DATA.json
├── imdb_movies copy.json
├── imdb_movies.json
├── readme.md
├── tmdb_5000_credits.json
```

### Key Files and Screenshots

#### Frontend

**index.html**

```html
<!DOCTYPE html>
<html>
<head>
  <base href="$FLUTTER_BASE_HREF">
  <meta charset="UTF-8">
  <meta content="IE=Edge" http-equiv="X-UA-Compatible">
  <meta name="description" content="A new Flutter project.">
  <meta name="apple-mobile-web-app-capable" content="yes">
  <meta name="apple-mobile-web-app-status-bar-style" content="black">
  <meta name="apple-mobile-web-app-title" content="infs803_group7_frontend">
  <link rel="apple-touch-icon" href="icons/Icon-192.png">
  <link rel="icon" type="image/png" href="favicon.png"/>
  <title>infs803_group7_frontend</title>
  <link rel="manifest" href="manifest.json">
  <script>
    const serviceWorkerVersion = null;
  </script>
  <script src="flutter.js" defer></script>
</head>
<body>
  <script>
    window.addEventListener('load', function(ev) {
      _flutter.loader.loadEntrypoint({
        serviceWorker: {
          serviceWorkerVersion: serviceWorkerVersion,
        },
        onEntrypointLoaded: function(engineInitializer) {
          engineInitializer.initializeEngine().then(function(appRunner) {
            appRunner.runApp();
          });
        }
      });
    });
  </script>
</body>
</html>
```

**manifest.json**

```json
{
    "name": "infs803_group7_frontend",
    "short_name": "infs803_group7_frontend",
    "start_url": ".",
    "display": "standalone",
    "background_color": "#0175C2",
    "theme_color": "#0175C2",
    "description": "A new Flutter project.",
    "orientation": "portrait-primary",
    "prefer_related_applications": false,
    "icons": [
        {
            "src": "icons/Icon-192.png",
            "sizes": "192x192",
            "type": "image/png"
        },
        {
            "src": "icons/Icon-512.png",
            "sizes": "512x512",
            "type": "image/png"
        },
        {
            "src": "icons/Icon-maskable-192.png",
            "sizes": "192x192",
            "type": "image/png",
            "purpose": "maskable"
        },
        {
            "src": "icons/Icon-maskable-512.png",
            "sizes": "512x512",
            "type": "image/png",
            "purpose": "maskable"
        }
    ]
}
```

#### Backend

**main.rs**

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use crate::ServerConfig;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Favorite {
    pub names: String,
    #[serde(rename = "date_x")]
    pub date_x: String,
    pub score: i64,
    pub genre: String,
    pub overview: String,
    pub crew: String,
    #[serde(rename = "orig_title")]
    pub orig_title: String,
    pub status: String,
    #[serde(rename = "orig_lang")]
    pub orig_lang: String,
    #[serde(rename = "budget_x")]
    pub budget_x: f64,
    pub revenue: f64,
    pub country: String,
    pub deleted: bool,
}

pub async fn fetch_favorites(
    State(server_config): State<ServerConfig>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("favorites");
    let favorites = data.get::<Vec<Option<Favorite>>>().await;

    match favorites {
        Ok(_) => {
            let json_response = serde_json::json!({"favorites": favorites.as_ref().unwrap()});
            Ok((StatusCode::OK, Json(json_response)))
        }
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("list favorites fail: { }", e)})),
        )),
    }
}
```

**Cargo.toml**

```toml
[package]
name = "movie_explorer"
version = "0.1.0"
edition = "2018"

[dependencies]
axum = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_with = "1.5"
tokio = { version = "1", features = ["full"] }
firebase = "0.2"
```

---

This comprehensive README file provides a clear and detailed guide to your project, including installation instructions, quick start guide, features, tech stack, and other important information. It also includes key file structure and code snippets to help users understand and
