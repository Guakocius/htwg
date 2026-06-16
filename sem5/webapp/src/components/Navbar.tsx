import "../style/Navbar.css";

import { useNavigate } from "react-router";

function enableDarkmode() {
  document.body.classList.toggle("light-mode");
}

function GenerateButtons() {
  const navigate = useNavigate();
  const FILES = ["Index", "Register", "Login", "Game", "Leaderboard"];

  const handleNavigation = (f: string) => {
    sessionStorage.setItem(f, JSON.stringify(f));
    navigate(`/${f}`);
  };

  return (
    <nav id="top-navbar">
      {FILES.map((file) => {
        const fileLowerCase = file.toLowerCase();

        return (
          <button
            key={fileLowerCase}
            onClick={() => handleNavigation(fileLowerCase)}
          >
            {file}
          </button>
        );
      })}

      <button id="darkmode" onClick={enableDarkmode}>
        Darkmode
      </button>
    </nav>
  );
}

export default function Navbar() {
  return <GenerateButtons />;
}
