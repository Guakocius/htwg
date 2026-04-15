const FILES = ["Index", "Register", "Login", "Game", "Leaderboard"];

//let nav = document.getElementById("top-navbar");
let nav = document.createElement("nav");
nav.setAttribute("id", "top-navbar");

FILES.forEach(file => {
  const FILE_LOWER_CASE = file.toLowerCase();
  const a = document.createElement("a");
  const PATH = FILE_LOWER_CASE.includes("index") ? "/" :
    "/src/".concat(FILE_LOWER_CASE).concat(".html");
  a.setAttribute("href", PATH);
  a.textContent = file;
  nav.appendChild(a);
});

let darkModeButton = document.createElement("button");
darkModeButton.id = "darkmode";
darkModeButton.textContent = "Light Mode";
nav.appendChild(darkModeButton);

document.body.prepend(nav);



