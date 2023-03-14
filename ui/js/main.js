// Array of all active window names
let names = [];

function loadMopidy(data) {
    if (checkWindowName(data.name) == false) {
        console.log("Name already in use");
        document.getElementById("status").innerText = "That name is already in use";
        return;
    }
    console.log(`${data.protocol}${data.address}:${data.port}${extension}`)
    let newWindow = new __TAURI__.window.WebviewWindow(data.name, {url: `${data.protocol}${data.address}:${data.port}${data.extension}`});
    console.log(newWindow);
    newWindow.once('tauri://created', function() {
        __TAURI__.invoke("add_menu_entry", {name: data.name});
        names.push(data.name);
        document.getElementById("status").innerText = "";
    })
}

function makeURLSafe(data) {
    data.protocol = data.protocol.toLowerCase() + "://";
    if (!data.extension.startsWith("/")) {
        data.extension = "/" + data.extension;
    };
    return data;
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

document.addEventListener("DOMContentLoaded", (ev) => {
    __TAURI__.invoke("request_connections").then(data =>{
        console.log(data);
        data.forEach(connection => {
            addConnection(connection);
        });
    });
});

function addConnection(data) {
    data = makeURLSafe(data);
    let connection = document.createElement("div");
    connection.classList.add("connection");

    let header = document.createElement("h3");
    header.innerText = data.name;
    connection.appendChild(header);

    let path = document.createElement("p");
    path.innerText = `${data.protocol}${data.address}:${data.port}${data.extension}`;
    connection.appendChild(path);

    let deleteIcon = document.createElement("p");
    deleteIcon.innerText = "🗑";
    deleteIcon.classList.add("deleteIcon");
    deleteIcon.title = "Delete";
    connection.appendChild(deleteIcon);

    connection.addEventListener("click", (ev) => {
        loadMopidy(data);
    });

    deleteIcon.addEventListener("click", (ev) => {
        ev.stopImmediatePropagation();
        console.log("Deleting an entry");
        __TAURI__.invoke("delete_connection", {name: data.name});
        connection.remove();
    })

    document.getElementById("connectionContainer").appendChild(connection);
}

function createConnection() {
    let protocol = document.getElementById("protocol").value;
    let address = document.getElementById("address").value;
    let port = document.getElementById("port").value;
    let extension = document.getElementById("extension").value;
    let name = document.getElementById("name").value;
    let props = {protocol: protocol, address: address, port: port, extension: extension, name: name};
    
    //Add to config and show in window
    __TAURI__.invoke("add_connection", props);
    addConnection(props);
}