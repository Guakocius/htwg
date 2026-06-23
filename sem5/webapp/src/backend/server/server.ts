import express from "express";
import type { Request, Response } from "express";
import mongoose from "mongoose";
import cors from "cors";

const app = express();
const PORT: number = 5000;

app.use(cors({ origin: "http://localhost:5173" }));
app.use(express.json());

const mongoURI: string = "mongodb://127.0.0.1:27017/flappyBase";

mongoose
  .connect(mongoURI)
  .then(() => console.log("Successfully connected to MongoDB"))
  .catch((e) => console.error("MongoDB connection error:", e));

interface IUse {
  username: string;
  password?: string;
}

const userSchema = new mongoose.Schema<IUser>({
  username: { type: String, required: true, unique: true },
  password: { type: String, required: true },
});

const User = mongoose.model<IUser>("User", userSchema);

app.post(
  "/api/users/register",
  async (req: Request, res: Response): Promise<void> => {
    try {
      const { username, password } = req.body;

      const newUser = new User({ username, password });
      await newUser.save();

      res.status(201).json({ message: "User registration successfully." });
    } catch (e) {
      console.error(e);
      res.status(500).json({ message: "Server error saving user data." });
    }
  },
);

app.listen(PORT, () => {
  console.log(`Backend server running on port ${PORT}`);
});
