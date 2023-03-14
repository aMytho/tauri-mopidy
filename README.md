# Tauri-Mopidy

A desktop app to access mopidy extensions.

## About

Tauri-Mopidy is a way to listen to and control your mopidy service. It **does not** host any of the extensions itself. Instead, it connects to your mopidy server and displays it in a webview using Tauri. It will be useless without a mopidy server to connect to.

## Why?

Every browser uses `x` amount of ram and adds `y` ram as tabs are created. A webview window is smaller than a full browser and uses less resources. If I didn't have a browser open, it would be better to use this program instead of a full browser.

## Installation and Usage

Download and run the installer for your platform. Once complete enter your server information. The URL and port are required. The name is the unique name of the window. The extension input allows you to navigate to a specific extension by URL path. Ex. /iris for the Iris extension.
