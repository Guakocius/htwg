const FILES = ["Register", "Login", "Game", "Leaderboard"];

//let nav = document.getElementById("top-navbar");
let nav = document.createElement("nav");
nav.setAttribute("id", "top-navbar");

FILES.forEach(file => {
  const fileLowerCase = file.toLowerCase();
  const a = document.createElement("a");
  a.setAttribute("href", "/src/" + fileLowerCase + ".html");
  a.textContent = file;
  nav.appendChild(a);
});
document.body.prepend(nav);
