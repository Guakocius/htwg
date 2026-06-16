import { BrowserRouter, Routes, Route } from "react-router";
import flappyImg from "./assets/flappybird.png";
import "./style/App.css";
import Navbar from "./components/Navbar";
import Index from "./pages/Index";
import Register from "./pages/Register";
import Login from "./pages/Login";
import Game from "./pages/Game";
import Leaderboard from "./pages/Leaderboard";

export default function App() {
  return (
    <>
      <BrowserRouter>
        <Navbar />

        <div className="page-content">
          <Routes>
            <Route path="/index" element={<Index />} />
            <Route path="/register" element={<Register />} />
            <Route path="/login" element={<Login />} />
            <Route path="/game" element={<Game />} />
            <Route path="/leaderboard" element={<Leaderboard />} />
          </Routes>
        </div>
      </BrowserRouter>

      <header>
        <h1>
          Welcome to Flappy Bird
          <span className="special-text">Reimagined!</span>
        </h1>
      </header>
      <main id="center">
        <div className="flappy">
          <img
            src={flappyImg}
            id="flappy-img"
            width="auto"
            height="auto"
            alt="a yellow pixelated bird"
          />
        </div>
      </main>

      <article>
        <h2
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
          }}
        >
          <u>Lore</u>
        </h2>

        <div>
          <p>
            Long ago, a small yellow bird lived in a quiet world above endless
            green pipes. Unlike the other birds, Flappy was never meant to stay
            in a cage or rest on the ground. It dreamed of the open sky and of
            proving that even the smallest wings could overcome impossible
            obstacles. One day, the bird escaped captivity and began its journey
            through the pipe-filled land, where every flight became a test of
            courage, timing, and endurance. With each narrow gap it passed,
            Flappy came closer to freedom and to becoming a legend among birds.
          </p>
        </div>
      </article>

      <section>
        <h2>Features</h2>
        <p>
          Flappy Bird Reimagined keeps the simple challenge of the original game
          while adding its own style, atmosphere, and expanded world.
        </p>

        <ul>
          <li>Classic arcade gameplay</li>
          <li>New visual style</li>
          <li>Lore and worldbuilding</li>
          <li>Leaderboard competition</li>
        </ul>
      </section>
      <footer>&copy; 2026</footer>
    </>
  );
}
