import { useNavigate } from "react-router";
import LeaderboardTable from "../components/LeaderboardTable";

/**
 *
 */
export default function Leaderboard() {
  const navigate = useNavigate();

  return (
    <>
      <div>
        <button onClick={() => navigate(-1)}>go back</button>
        <LeaderboardTable />
      </div>
    </>
  );
}
