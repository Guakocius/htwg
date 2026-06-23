import express from "express";
import type { Request, Response, NextFunction } from "express";
import mongoose from "mongoose";
import cors from "cors";
import bcrypt from "bcrypt";
import jwt from "jsonwebtoken";
import dotenv from "dotenv";

dotenv.config();

const app = express();
const PORT = process.env.PORT || 5000;
const JWT_SECRET = process.env.JWT_SECRET || "secret-key";

app.use(
  cors({
    origin: "http://localhost:5173",
    credentials: true,
  }),
);
app.use(express.json());

const mongoURI: string = "mongodb://127.0.0.1:27017/flappyBirdDB";

mongoose
  .connect(mongoURI)
  .then(() => console.log("Successfully connected to MongoDB"))
  .catch((e) => console.error("MongoDB connection error:", e));

const userSchema = new mongoose.Schema<IUser>({
  username: { type: String, required: true, unique: true },
  password: { type: String, required: true },
});

const User = mongoose.model("User", userSchema);

interface AuthenticatedRequest extends Request {
  user?: any;
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
    const verified = jwt.verify(token, JWT_SECRET);
    req.user = verified;
    next();
  } catch (e) {
    res.status(403).json({ message: "Invalid or expired token." });
  }
};

app.post(
  "/api/users/register",
  async (req: Request, res: Response): Promise<void> => {
    try {
      const { username, password } = req.body;

      if (!username || !password) {
        res
          .status(400)
          .json({ message: "Username and password are required." });
        return;
      }

      const userExists = await User.findOne({ username });
      if (userExists) {
        res.status(400).json({ message: "Username is already taken." });
        return;
      }

      const salt = await bcrypt.genSalt(10);
      const hashedPassword = await bcrypt.hash(password, salt);

      const newUser = new User({ username, password: hashedPassword });

      await newUser.save();
      res.status(201).json({ message: "User registration successfully." });
    } catch (e) {
      console.error(e);
      res.status(500).json({ message: "Server error saving user data." });
    }
  },
);

app.post(
  "/api/users/login",
  async (req: Request, res: Response): Promise<void> => {
    try {
      const { username, password } = req.body;

      const user = await User.findOne({ username });
      if (!user) {
        res.status(400).json({ message: "Invalid username or password." });
        return;
      }

      const validPassword = await bcrypt.compare(password, user.password);
      if (!validPassword) {
        res.status(400).json({ message: "Invalid username or password" });
        return;
      }

      const token = jwt.sign(
        { id: user._id, username: user.username },
        JWT_SECRET,
        { expiresIn: "1h" },
      );

      res.status(200).json({
        message: "Login successful.",
        token,
        user: { id: user._id, username: user.username },
      });
    } catch (e) {
      console.error(e);
      res.status(500).json({ message: "Server error during login." });
    }
  },
);

app.get(
  "/api/game/leaderboard",
  authenticateToken,
  async (req: AuthenticatedRequest, res: Response) => {
    res.status(200).json({
      message: `Hello ${req.user.username}, here is the leaderboard of today.`,
    });
  },
);

app.listen(PORT, () => {
  console.log(`Backend server running on port ${PORT}`);
});
