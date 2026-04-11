const FILES = ["Register", "Login", "Game", "Leaderboard"];

let nav = document.getElementById("top-navbar");

FILES.forEach(file => {
  const a = document.createElement("a");
  a.setAttribute("href", "src/" + file.toLowerCase() + ".html");
  a.textContent = file;
  nav.appendChild(a);
});

