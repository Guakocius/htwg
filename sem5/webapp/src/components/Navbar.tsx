import "../style/Navbar.css";

function generateButtons() {
  const FILES = ["Index", "Register", "Login", "Game", "Leaderboard"];

  return (
    <><nav id="top-navbar">
      {FILES.map(file => {
        const fileLowerCase = file.toLowerCase();

        const path = fileLowerCase.includes("index") ? "/" : `/${fileLowerCase}.html`;

        return (
          <a key={file} href={path}>{file}</a>
        )
      })}

      <button id="darkmode">Darkmode</button>
    </nav>

    </>
  )
}
export default function Navbar() {
  return (
    <>{generateButtons()}</>
  );
}
