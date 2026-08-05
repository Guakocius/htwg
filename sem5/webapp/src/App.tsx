import { BrowserRouter, Routes, Route } from "react-router";
import "./style/App.css";
import Navbar from "./frontend/components/Navbar";
import Index from "./frontend/pages/Index";
import Register from "./frontend/pages/Register";
import Login from "./frontend/pages/Login";
import Game from "./frontend/pages/Game";
import Leaderboard from "./frontend/pages/Leaderboard";

/**
 * Main application component for the landing page.
 * Displays the Flappy Bird title with its iconic image
 * and the Navbar.
 */
export default function App() {
  return (
    <BrowserRouter>
      <Navbar />

      <div className="page-content">
        <Routes>
          <Route path="/" element={<Index />} />
          <Route path="/register" element={<Register />} />
          <Route path="/login" element={<Login />} />
          <Route path="/game" element={<Game />} />
          <Route path="/leaderboard" element={<Leaderboard />} />
        </Routes>
      </div>
    </BrowserRouter>
  );
}
