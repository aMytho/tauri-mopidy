// Connection information
let protocol = "http://"
let IP = "localhost";
let port = ":6680";
let extension = "";
let windowName = "Mopidy";
// Array of all active window names
let names = [];

function loadMopidy() {
    if (checkWindowName(windowName) == false) {
        console.log("Name already in use");
        document.getElementById("status").innerText = "That name is already in use";
        return;
    }
    let newWindow = new __TAURI__.window.WebviewWindow(windowName, {url: protocol + IP + port + extension});
    console.log(newWindow)
    newWindow.once('tauri://created', function() {
        __TAURI__.invoke("add_menu_entry", {name: windowName});
        names.push(windowName);
        document.getElementById("status").innerText = "";
    })
}

function updatePort(newPort) {
    port = ":" + newPort
}

function updateIP(newIP) {
    IP = newIP
}

function updateName(newName) {
    windowName = newName;
}

function updateProtocol(newProtocol) {
    protocol = newProtocol;
}

function updateExtension(newExtension) {
    extension = newExtension;
}

function checkWindowName(name) {
    if (name == "main") return false;
    if (names.includes(name)) return false;
    return isAlphaNumeric(name);
}

function isAlphaNumeric(str) {
    var code, i, len;

    for (i = 0, len = str.length; i < len; i++) {
      code = str.charCodeAt(i);
      if (!(code > 47 && code < 58) && // numeric (0-9)
          !(code > 64 && code < 91) && // upper alpha (A-Z)
          !(code > 96 && code < 123)) { // lower alpha (a-z)
        return false;
      }
    }
    return true;
  };

__TAURI__.event.listen("closed", function(window) {
    console.log(window);
    names = names.filter((name) => {
        return name != window.windowLabel;
    })
})