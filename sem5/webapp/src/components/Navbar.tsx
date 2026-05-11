import "../style/Navbar.css";

function enableDarkmode() {
  document.body.classList.toggle("light-mode");
}

function generateButtons() {
  const FILES = ["Index", "Register", "Login", "Game", "Leaderboard"];

  return (
    <><nav id="top-navbar">
      {FILES.map(file => {
        //const fileLowerCase = file.toLowerCase();

        // TODO: Change path in M3 with React Router DOM
        const path = "";// fileLowerCase.includes("index") ? "/" : `/${fileLowerCase}.html`;

        return (
          <a key={file} href={path}>{file}</a>
        )
      })}

      <button id="darkmode" onClick={enableDarkmode}>Darkmode</button>
    </nav>

    </>
  )
}
export default function Navbar() {
  return (
    <>{generateButtons()}</>
  );
}
