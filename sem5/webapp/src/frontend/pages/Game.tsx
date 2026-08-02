import { useState, useEffect } from "react";
import axios from "axios";

interface Pipe {
  x: number;
  topHeight: number;
  passed: boolean;
}

/**
 * Page for displaying the game and handling the game's logic.
 */
export default function Game() {
  const [birdPosition, setBirdPosition] = useState<number>(250);
  const [velocity, setVelocity] = useState<number>(0);
  const [score, setScore] = useState<number>(0);
  const [gameOver, setGameOver] = useState<boolean>(false);
  const [gameStarted, setGameStarted] = useState<boolean>(false);
  const [pipes, setPipes] = useState<Pipe[]>([]);

  const gameHeight = 500;
  const gameWidth = 400;
  const gravity = 0.6;
  const jumpStrength = -8.5;
  const pipeWidth = 60;
  const pipeGap = 120;
  const pipeSpeed = 3;

  const handleJump = () => {
    if (!gameStarted) {
      setGameStarted(true);
    }
    if (!gameOver) {
      setVelocity(jumpStrength);
    }
  };

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.code === "Space") {
        handleJump();
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [gameStarted, gameOver]);

  useEffect(() => {
    if (!gameStarted || gameOver) return;

    const gameInterval = setInterval(() => {
      setBirdPosition((prev) => {
        const nextPos = prev + velocity;
        if (nextPos >= gameHeight - 30 || nextPos <= 0) {
          endGame();
        }
        return nextPos;
      });
      setVelocity((prev) => prev + gravity);

      setPipes((prevPipes) => {
        const updatedPipes = prevPipes
          .map((pipe) => {
            const nextX = pipe.x - pipeSpeed;

            if (!pipe.passed && nextX + pipeWidth < 100) {
              setScore((s) => s + 1);
              return { ...pipe, x: nextX, passed: true };
            }
            return { ...pipe, x: nextX };
          })
          .filter((pipe) => pipe.x > -pipeWidth);

        const lastPipe = updatedPipes[updatedPipes.length - 1];
        if (!lastPipe || lastPipe.x < gameWidth - 200) {
          const randomTopHeight =
            Math.floor(Math.random() * (gameHeight - pipeGap - 100)) + 40;
          updatedPipes.push({
            x: gameWidth,
            topHeight: randomTopHeight,
            passed: false,
          });
        }

        return updatedPipes;
      });

      pipes.forEach((pipe) => {
        const birdX = 100;
        const birdY = birdPosition;
        const birdSize = 30;

        if (birdX + birdSize > pipe.x && birdX < pipe.x + pipeWidth) {
          if (
            birdY < pipe.topHeight ||
            birdY + birdSize > pipe.topHeight + pipeGap
          ) {
            endGame();
          }
        }
      });
    }, 24);

    return () => clearInterval(gameInterval);
  }, [gameStarted, gameOver, velocity, birdPosition, pipes]);

  const endGame = () => {
    setGameOver(true);
    saveScore(score);
  };

  const saveScore = async (finalScore: number) => {
    const token = sessionStorage.getItem("token");
    if (!token) return;

    try {
      await axios.post(
        "http://localhost:5000/api/game/score",
        { score: finalScore },
        { headers: { Authorization: `Bearer ${token}` } },
      );
    } catch (e) {
      console.error("Failed to save high score:", e);
    }
  };

  const restartGame = () => {
    setBirdPosition(250);
    setVelocity(0);
    setScore(0);
    setPipes([]);
    setGameOver(false);
    setGameStarted(true);
  };

  return (
    <div
      onClick={handleJump}
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        userSelect: "none",
      }}
    >
      <header
        id="game-points"
        style={{ fontSize: "2rem", marginBottom: "10px" }}
      >
        Score: {score}
      </header>
      <main
        style={{
          position: "relative",
          width: `${gameWidth}px`,
          height: `${gameHeight}px`,
          backgroundColor: "#70c5ce",
          overflow: "hidden",
          border: "2px solid #000",
        }}
      >
        <div
          id="player-container"
          style={{
            position: "absolute",
            left: "100px",
            top: `${birdPosition}px`,
            width: "30px",
            height: "30px",
          }}
        >
          <img
            src="assets/img/flappybird.png"
            alt="Flappy Bird"
            style={{ width: "100%", height: "100%" }}
          />
        </div>
        <div id="pipe-container">
          {pipes.map((pipe, index) => (
            <div key={index}>
              <div
                style={{
                  position: "absolute",
                  left: `${pipe.x}px`,
                  top: 0,
                  width: `${pipeWidth}px`,
                  height: `${pipe.topHeight}px`,
                  backgroundColor: "green",
                  border: "3px solid black",
                }}
              />
              <div
                style={{
                  position: "absolute",
                  left: `${pipe.x}px`,
                  top: `${pipe.topHeight + pipeGap}px`,
                  width: `${pipeWidth}px`,
                  height: `${gameHeight - pipe.topHeight - pipeGap}px`,
                  backgroundColor: "green",
                  border: "2px solid black",
                }}
              />
            </div>
          ))}
        </div>

        {!gameStarted && (
          <div
            style={{ textAlign: "center", marginTop: "200px", color: "#fff" }}
          >
            <h2>Click or Press Space to Start</h2>
          </div>
        )}

        {gameOver && (
          <div
            style={{
              position: "absolute",
              top: "30%",
              width: "100%",
              textAlign: "center",
              backgroundColor: "rgba(0, 0, 0, 0.7)",
              color: "#fff",
              padding: "20px 0",
            }}
          >
            <h2>Game Over</h2>
            <button
              onClick={restartGame}
              style={{
                padding: "10px 20px",
                fontSize: "1rem",
                cursor: "pointer",
              }}
            >
              Play Again
            </button>
          </div>
        )}
      </main>
    </div>
  );
}
