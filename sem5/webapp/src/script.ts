const FILES = ["Register", "Login", "Game", "Leaderboard"];

let nav: HTMLElement = document.getElementById("top-navbar") as HTMLElement;

FILES.forEach(file => {
  const linkButton = document.createElement("button");

  const fileLowerCase = file.toLowerCase();
  const a = document.createElement("a");
  a.setAttribute("href", "src/" + fileLowerCase + ".html");
  a.textContent = file;
  //linkButton.textContent = file;
  //linkButton.setAttribute("type", "button");
  //linkButton.setAttribute("onclick", "location.href='" + "./src/" + fileLowerCase + ".html'");
  nav.appendChild(a);
});

