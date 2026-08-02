import { useEffect, useState } from "react";
import axios from "axios";

interface LeaderboardEntry {
  _id: string;
  username: string;
  highScore: number;
}

/**
 * Displays user scores fetched from the database.
 */
export default function LeaderboardTable() {
  const [scores, setScores] = useState<LeaderboardEntry[]>([]);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    const fetchLeaderboard = async () => {
      try {
        const response = await axios.get(
          "http://localhost:5000/api/game/leaderboard",
        );
        setScores(response.data);
      } catch (e) {
        console.error("Error fetching leaderboard:", e);
      } finally {
        setLoading(false);
      }
    };

    fetchLeaderboard();
  }, []);

  return (
    <div>
      {loading ? (
        <p>Loading leaderboard...</p>
      ) : (
        <table id="leaderboard">
          <thead>
            <tr>
              <th>Rank</th>
              <th>Username</th>
              <th>Score</th>
            </tr>
          </thead>
          <tbody>
            {scores.map((entry, index) => (
              <tr key={entry._id}>
                <td>{index + 1}</td>
                <td className="name">{entry.username}</td>
                <td className="score">{entry.highScore}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}
