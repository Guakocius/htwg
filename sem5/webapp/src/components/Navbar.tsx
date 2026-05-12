import "../style/Navbar.css";

import { Route, Routes, useNavigate } from "react-router";
import { useEffect } from "react";

function enableDarkmode() {
  document.body.classList.toggle("light-mode");
}

const NavbarRoutes = (f: string) => {
  const navigate = useNavigate();

  useEffect(() => {
    navigate(`/${f}`);
  });

  return (
    <Routes>
      <Route path="/" />
      <Route path={`/${f}`} />
    </Routes>
  );
};

function GenerateButtons() {
  const FILES = ["Index", "Register", "Login", "Game", "Leaderboard"];

  return (
    <>
      <nav id="top-navbar">
        {FILES.map((file) => {
          const fileLowerCase = file.toLowerCase();

          sessionStorage.setItem(fileLowerCase, JSON.stringify(fileLowerCase));

          return (
            <button
              key={fileLowerCase}
              onClick={() => NavbarRoutes(fileLowerCase)}
            >
              {file}
            </button>
          );
        })}

        <button id="darkmode" onClick={enableDarkmode}>
          Darkmode
        </button>
      </nav>
    </>
  );
}
export default function Navbar() {
  const buttons = GenerateButtons();
  return <>{buttons}</>;
}
