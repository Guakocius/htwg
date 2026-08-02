import express from "express";
import type { Request, Response, NextFunction } from "express";
import cors from "cors";
import bcrypt from "bcrypt";
import jwt from "jsonwebtoken";
import dotenv from "dotenv";
import Database from "better-sqlite3";
import fs from "fs";
import { fileURLToPath } from "url";
import path from "path";

dotenv.config();

const app = express();
const PORT = process.env.PORT || 5000;
const JWT_SECRET = process.env.JWT_SECRET || "secret-key";

app.use(
  cors({
    origin: ["http://localhost:5173", "http://127.0.0.1:5173"],
    credentials: true,
    methods: ["GET", "POST", "PUT", "DELETE", "OPTIONS"],
    allowedHeaders: ["Content-Type", "Authorization"],
  }),
);
app.use(express.json());

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const dbDir = path.join(__dirname, "../../../data");
if (!fs.existsSync(dbDir)) {
  fs.mkdirSync(dbDir, { recursive: true });
}

const db = new Database(path.join(dbDir, "flappybird.db"));

db.exec(`
        CREATE TABLE IF NOT EXISTS users (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          username TEXT UNIQUE NOT NULL,
          password TEXT NOT NULL,
          highScore INTEGER DEFAULT 0
        )
`);

console.log("Connected to SQLite successfully.");

interface UserPayload {
  id: number;
  username: string;
}

interface AuthenticatedRequest extends Request {
  user?: UserPayload | jwt.JwtPayload;
}

const authenticateToken = (
  req: AuthenticatedRequest,
  res: Response,
  next: NextFunction,
): void => {
  const authHeader = req.headers["authorization"];
  const token = authHeader && authHeader.split(" ")[1];

  if (!token) {
    res.status(401).json({ message: "Access denied. No token provided." });
    return;
  }

  try {
    const verified = jwt.verify(token, JWT_SECRET) as UserPayload;
    req.user = verified;
    next();
  } catch (e) {
    res.status(403).json({ message: `Invalid or expired token: ${e}` });
  }
};

app.post("/api/users/register", async (req: Request, res: Response) => {
  try {
    const { username, password } = req.body;

    if (!username || !password) {
      return res
        .status(400)
        .json({ message: "Username and password are required." });
    }

    const userExists = db
      .prepare("SELECT * FROM users WHERE username = ?")
      .get(username);

    if (userExists) {
      return res.status(400).json({ message: "Username is already taken." });
    }

    const salt = await bcrypt.genSalt(10);
    const hashedPassword = await bcrypt.hash(password, salt);

    const stmt = db.prepare(
      "INSERT INTO users (username, password) VALUES (?, ?)",
    );
    stmt.run(String(username), String(hashedPassword));

    return res.status(201).json({ message: "User registration successful." });
  } catch (e) {
    console.error("Registration error:", e);
    return res.status(500).json({ message: "Server error saving user data." });
  }
});

app.post("/api/users/login", async (req: Request, res: Response) => {
  try {
    const { username, password } = req.body;

    const user = db
      .prepare("SELECT * FROM users WHERE username = ?")
      .get(username) as
      | { id: number; username: string; password: string; highScore: number }
      | undefined;

    if (!user) {
      return res.status(400).json({ message: "Invalid username or password." });
    }

    const validPassword = bcrypt.compareSync(password, user.password);
    if (!validPassword) {
      return res.status(400).json({ message: "Invalid username or password" });
    }

    const token = jwt.sign(
      { id: user.id, username: user.username },
      JWT_SECRET,
      { expiresIn: "1h" },
    );

    return res.status(200).json({
      message: "Login successful.",
      token,
      user: {
        id: user.id,
        username: user.username,
        highScore: user.highScore,
      },
    });
  } catch (e) {
    console.error("Login error:", e);
    res.status(500).json({ message: "Server error during login." });
  }
});

app.get("/api/game/leaderboard", async (_req: Request, res: Response) => {
  try {
    const leaderboard = db
      .prepare(
        "SELECT id as _id, username, highScore FROM users ORDER BY highScore DESC LIMIT 10",
      )
      .all();

    return res.status(200).json(leaderboard);
  } catch (e) {
    console.error("Leaderboard error:", e);
    res.status(500).json({ message: "Server error fetching leaderboard." });
  }
});

app.post(
  "/api/game/score",
  authenticateToken,
  async (req: AuthenticatedRequest, res: Response) => {
    try {
      const { score } = req.body;
      const userId = req.user?.id;

      if (!userId) {
        res.status(401).json({ message: "Unauthorized." });
        return;
      }

      const user = db
        .prepare("SELECT * FROM users WHERE id = ?")
        .get(userId) as
        | { id: number; username: string; highScore: number }
        | undefined;

      if (!user) {
        res.status(404).json({ message: "User not found." });
        return;
      }

      if (score > user.highScore) {
        db.prepare("UPDATE users SET highScore = ? WHERE id = ?").run(
          score,
          userId,
        );
        return res
          .status(200)
          .json({ message: "New high score!", highScore: user.highScore });
      }

      return res
        .status(200)
        .json({ message: "Score processed.", highScore: user.highScore });
    } catch (e) {
      console.error("Score error:", e);
      res.status(500).json({ message: "Server error updating score." });
    }
  },
);

app.listen(PORT, () => {
  console.log(`Backend server running on port ${PORT}`);
});
